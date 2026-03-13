# SQL Security Analysis: Rust Todo App

SQL injection analysis of the Rust todo app built on the Generic Temper ORM. This analysis focuses exclusively on SQL generation and execution — the core value proposition of the ORM.

**Analysis Date:** 2026-03-12
**Framework:** Axum + rusqlite + askama
**ORM:** Generic Temper ORM (compiled to Rust)

---

## How This App Uses the ORM

All user-facing CRUD operations flow through the Temper ORM's type-safe SQL generation:

| Operation | Method | SQL Source |
|-----------|--------|------------|
| SELECT lists/todos | `from(safe_identifier("...")?).where(...).to_sql()` | ORM |
| INSERT list/todo | `changeset(table, params).cast(fields).validate_required(fields).to_insert_sql()` | ORM |
| UPDATE todo title | `changeset(table, params).cast(fields).to_update_sql(id)` | ORM |
| UPDATE completed | `changeset(table, params).cast(fields).to_update_sql(id)` | ORM |
| DELETE list/todo | `delete_sql(table, id)` | ORM |
| WHERE clauses | `SqlBuilder::append_safe()` + `append_int32()` | ORM |
| Cascade delete | `DELETE FROM todos WHERE list_id = ?1` | Raw (parameterized) |
| JOIN + aggregate | `SELECT l.*, COUNT(t.id) ... LEFT JOIN ...` | Raw (hardcoded — no user input) |
| DDL | `CREATE TABLE IF NOT EXISTS ...` | Raw (static) |

### ORM SQL Generation Path

```
User input (form field)
  → Axum Form<T> deserialization
    → Temper Map construction
      → changeset(table_def, params_map)         [factory — sealed interface]
        → .cast(allowed_fields)                   [SafeIdentifier whitelist]
          → .validate_required(fields)            [non-null enforcement]
            → .to_insert_sql()?                   [type-dispatched escaping]
              → .to_string()                      [rendered SQL string]
                → conn.execute(&sql, [])           [rusqlite execution]
```

For queries:
```
Route parameter (Path<i64>)
  → Axum typed extraction (guaranteed i64)
    → id as i32                                   [truncation — see RS-SQL-1]
      → SqlBuilder::append_int32(id)              [bare integer]
        → from(safe_identifier("lists")?).where(fragment).to_sql()
          → conn.prepare(&sql)?.query_map([], ...)
```

---

## SQL Injection Analysis

### ORM-Generated SQL: Protected

**SafeIdentifier validation** — Rust's `Result` type makes `safe_identifier()` returns explicit — the `?` operator propagates failures. All identifiers validated against `[a-zA-Z_][a-zA-Z0-9_]*`.

**SqlString escaping** — String values pass through `SqlString` which escapes `'` → `''`.

**Changeset field whitelisting** — `cast()` restricts columns in INSERT/UPDATE. This app uses the ORM for toggle too (via `changeset().to_update_sql()`), unlike most other apps which use raw SQL for toggle.

**Type safety** — Rust's ownership model ensures `SqlBuilder` parts cannot be modified after construction. The `SqlFragment` is consumed once and rendered.

### Raw SQL: Mixed

```rust
// Cascade delete — parameterized with ?1
conn.execute("DELETE FROM todos WHERE list_id = ?1", params![list_id])?;

// JOIN aggregate — hardcoded, no user input
conn.prepare("SELECT l.id, l.name, COUNT(t.id) ... FROM lists l LEFT JOIN todos t ...")?;

// Seed data — hardcoded literals
conn.execute("INSERT INTO todos (title, list_id) VALUES (?1, ?2)", params![title, id])?;
```

The JOIN query has no user input — it returns all lists with counts. The cascade delete uses rusqlite's `params!` macro for safe binding.

### DDL: Static

Schema creation uses hardcoded `CREATE TABLE` statements.

---

## Findings

| # | Severity | CWE | Finding |
|---|----------|-----|---------|
| RS-SQL-1 | LOW | CWE-681 | `Path<i64>` route params are cast to `i32` via `id as i32` for ORM calls. IDs above `i32::MAX` (2,147,483,647) silently wrap, potentially targeting a different record. SQLite uses `i64` rowids, so the ORM's `Int32` type creates a mismatch. Not exploitable for injection, but could cause incorrect deletes/updates. |
| RS-SQL-2 | INFO | CWE-89 | ORM SQL is executed via `conn.execute(&sql, [])` with an empty parameter array — the SQL is pre-rendered with escaped values. Correct, but parameterized would be strictly safer. |
| RS-SQL-3 | INFO | CWE-400 | SELECT queries use `to_sql()` without limits. The ORM's `safe_to_sql(default_limit)` is available but unused. |

### ORM-Level Concerns (shared across all apps)

| # | Severity | CWE | Finding |
|---|----------|-----|---------|
| ORM-1 | MEDIUM | CWE-89 | `to_insert_sql`/`to_update_sql` pass `pair.key` (a `String`) to `append_safe`. Safe by construction but not type-enforced at the call site. |
| ORM-2 | LOW | CWE-89 | `SqlDate.format_to` wraps `value.to_string()` in quotes without escaping. |
| ORM-3 | LOW | CWE-20 | `SqlFloat64.format_to` can produce `NaN`/`Infinity`. |

---

## Verdict

**No SQL injection vulnerabilities found.** The ORM covers all user-input-to-SQL paths. This is the only app that uses the ORM for toggle-completed (rather than raw SQL), giving it the highest ORM coverage. The `i64` → `i32` truncation is a correctness concern, not an injection vector.
