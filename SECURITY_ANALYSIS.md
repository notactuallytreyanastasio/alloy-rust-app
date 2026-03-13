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

---

## Evolution: Temper-Level Remediation

**Date:** 2026-03-12
**Commit:** [`1df8c7a`](https://github.com/notactuallytreyanastasio/generic_orm/commit/1df8c7a)

The security analysis above identified 3 ORM-level concerns (ORM-1, ORM-2, ORM-3) shared across all 6 app implementations. Because the ORM is written once in Temper and compiled to all backends, fixing these issues at the Temper source level automatically resolves them in every language — including this Rust app.

### What Changed

**ORM-1 (MEDIUM → RESOLVED): Column name type safety in INSERT/UPDATE SQL**

The `to_insert_sql()` and `to_update_sql()` methods previously passed `pair.key` (a raw `String`) to `append_safe()`. While safe by construction (keys originated from `cast()` via `SafeIdentifier::sql_value()`), the type system didn't enforce this. A future refactor could have silently introduced an unvalidated code path.

The fix routes column names through the looked-up `FieldDef.name.sql_value()` — a `SafeIdentifier` — so the column name in the generated SQL always comes from a validated identifier, not a raw map key.

**ORM-2 (LOW → RESOLVED): SqlDate quote escaping**

`SqlDate::format_to()` previously wrapped `value.to_string()` in quotes without escaping. The fix adds character-by-character quote escaping identical to `SqlString::format_to()`, ensuring defense-in-depth against any future Date format that might contain single quotes.

**ORM-3 (LOW → RESOLVED): SqlFloat64 NaN/Infinity handling**

`SqlFloat64::format_to()` previously called `value.to_string()` directly, which could produce `NaN`, `inf`, or `-inf` — none valid SQL literals. The fix checks for these values and renders `NULL` instead, which is the safest SQL representation for non-representable floating-point values.

### Why This Matters

This is the core value proposition of a cross-compiled ORM: **one fix in Temper source propagates to all 6 backends simultaneously.** The same commit that fixed the Rust compiled output also fixed JavaScript, Python, Java, Lua, and C#. No per-language patches needed. No risk of inconsistent fixes across implementations.

### Updated Status

| Finding | Original | Current | Resolution |
|---------|----------|---------|------------|
| ORM-1 | MEDIUM | RESOLVED | Column names routed through `SafeIdentifier` |
| ORM-2 | LOW | RESOLVED | `SqlDate::format_to()` now escapes quotes |
| ORM-3 | LOW | RESOLVED | `SqlFloat64::format_to()` renders NaN/Infinity as `NULL` |
| ORM-4 | INFO | ACKNOWLEDGED | Design limitation — escaping-based, not parameterized |
