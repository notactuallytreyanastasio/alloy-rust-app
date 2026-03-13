use askama::Template;
use axum::{
    extract::{Path, Query as AxumQuery, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use orm::src::{
    avg_col, changeset, col, count_all, count_col, delete_from, delete_sql, except_sql,
    exists_sql, from, intersect_sql, max_col, min_col, safe_identifier, subquery, sum_col,
    union_all_sql, union_sql, update, ChangesetTrait, FieldDef, FieldType,
    ForShare, ForUpdate, IntField, LockMode, NullsFirst, NullsLast,
    NullsPosition, NumberValidationOpts, SafeIdentifier, SqlBuilder, SqlFragment, SqlInt32,
    SqlPart, SqlString, StringField, TableDef,
};
use rusqlite::Connection;
use serde::Deserialize;
use std::sync::{Arc, Mutex};
use temper_core::ToList;
use tower_http::services::ServeDir;

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<Connection>>,
    // Table definitions
    list_table: TableDef,
    todo_table: TableDef,
    #[allow(dead_code)]
    tag_table: TableDef,
    #[allow(dead_code)]
    todo_tag_table: TableDef,
    // Pre-built SafeIdentifiers
    sid_id: SafeIdentifier,
    sid_name: SafeIdentifier,
    sid_description: SafeIdentifier,
    sid_title: SafeIdentifier,
    sid_completed: SafeIdentifier,
    sid_priority: SafeIdentifier,
    sid_due_date: SafeIdentifier,
    sid_list_id: SafeIdentifier,
    #[allow(dead_code)]
    sid_todo_id: SafeIdentifier,
    #[allow(dead_code)]
    sid_tag_id: SafeIdentifier,
    sid_created_at: SafeIdentifier,
    // Table name identifiers
    sid_lists: SafeIdentifier,
    sid_todos: SafeIdentifier,
    sid_tags: SafeIdentifier,
    sid_todo_tags: SafeIdentifier,
}

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct TodoList {
    id: i64,
    name: String,
    description: Option<String>,
    #[allow(dead_code)]
    created_at: Option<String>,
}

#[derive(Debug)]
struct TodoItem {
    id: i64,
    title: String,
    completed: bool,
    priority: i64,
    due_date: Option<String>,
    #[allow(dead_code)]
    list_id: i64,
    #[allow(dead_code)]
    created_at: Option<String>,
}

struct ListWithCount {
    id: i64,
    name: String,
    todo_count: i64,
}

struct StatRow {
    label: String,
    value: String,
}

struct SearchResult {
    #[allow(dead_code)]
    id: i64,
    title: String,
    list_name: String,
}

struct CombinedRow {
    title: String,
    #[allow(dead_code)]
    source: String,
}

#[allow(dead_code)]
struct TagInfo {
    id: i64,
    name: String,
}

// ---------------------------------------------------------------------------
// Templates
// ---------------------------------------------------------------------------

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    lists: Vec<ListWithCount>,
    sql_used: String,
}

#[derive(Template)]
#[template(path = "list.html")]
struct ListTemplate {
    list: TodoList,
    todos: Vec<TodoItem>,
    completed_count: usize,
    sql_used: String,
}

#[derive(Template)]
#[template(path = "search.html")]
struct SearchTemplate {
    query: String,
    results: Vec<SearchResult>,
    sql_used: String,
}

#[derive(Template)]
#[template(path = "stats.html")]
struct StatsTemplate {
    stats: Vec<StatRow>,
    per_list_stats: Vec<StatRow>,
    sql_queries: Vec<String>,
}

#[derive(Template)]
#[template(path = "high_priority.html")]
struct HighPriorityTemplate {
    list: TodoList,
    todos: Vec<TodoItem>,
    sql_used: String,
}

#[derive(Template)]
#[template(path = "overdue.html")]
struct OverdueTemplate {
    todos: Vec<TodoItem>,
    sql_used: String,
}

#[derive(Template)]
#[template(path = "combined.html")]
struct CombinedTemplate {
    union_rows: Vec<CombinedRow>,
    intersect_rows: Vec<CombinedRow>,
    except_rows: Vec<CombinedRow>,
    sql_queries: Vec<String>,
}

#[derive(Template)]
#[template(path = "subquery.html")]
struct SubqueryTemplate {
    list: TodoList,
    has_completed: bool,
    completed_todos: Vec<TodoItem>,
    sql_queries: Vec<String>,
}

#[derive(Template)]
#[template(path = "paginated.html")]
struct PaginatedTemplate {
    list: TodoList,
    todos: Vec<TodoItem>,
    page: i32,
    total_pages: i32,
    total_count: i64,
    sql_queries: Vec<String>,
}

#[derive(Template)]
#[template(path = "locks.html")]
struct LocksTemplate {
    for_update_sql: String,
    for_share_sql: String,
}

#[derive(Template)]
#[template(path = "joins.html")]
struct JoinsTemplate {
    results: Vec<StatRow>,
    sql_queries: Vec<String>,
}

#[derive(Template)]
#[template(path = "changeset_demo.html")]
struct ChangesetDemoTemplate {
    results: Vec<StatRow>,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate {
    message: String,
}

// ---------------------------------------------------------------------------
// Form types
// ---------------------------------------------------------------------------

#[derive(Deserialize)]
struct CreateListForm {
    name: String,
    #[serde(default)]
    description: String,
}

#[derive(Deserialize)]
struct CreateTodoForm {
    title: String,
    #[serde(default)]
    priority: String,
    #[serde(default)]
    due_date: String,
}

#[derive(Deserialize)]
struct EditTodoForm {
    title: String,
}

#[derive(Deserialize)]
struct SearchQuery {
    #[serde(default)]
    q: String,
}

#[derive(Deserialize)]
struct BulkCompleteForm {
    #[serde(default)]
    ids: String,
}

// ---------------------------------------------------------------------------
// Error handling
// ---------------------------------------------------------------------------

struct AppError(Box<dyn std::error::Error + Send + Sync>);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let template = ErrorTemplate {
            message: format!("{}", self.0),
        };
        match template.render() {
            Ok(html) => (StatusCode::INTERNAL_SERVER_ERROR, Html(html)).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Something went wrong: {}", self.0),
            )
                .into_response(),
        }
    }
}

impl<E> From<E> for AppError
where
    E: Into<Box<dyn std::error::Error + Send + Sync>>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// ---------------------------------------------------------------------------
// ORM schema definitions
// ---------------------------------------------------------------------------

fn sid(name: &str) -> SafeIdentifier {
    safe_identifier(name).expect(&format!("invalid identifier: {}", name))
}

fn build_list_table() -> TableDef {
    TableDef::new(
        sid("lists"),
        [
            FieldDef::new(sid("name"), FieldType::new(StringField::new()), false, None, false),
            FieldDef::new(sid("description"), FieldType::new(StringField::new()), true, None, false),
            FieldDef::new(sid("created_at"), FieldType::new(StringField::new()), true, None, false),
        ],
        Some(sid("id")),
    )
}

fn build_todo_table() -> TableDef {
    TableDef::new(
        sid("todos"),
        [
            FieldDef::new(sid("title"), FieldType::new(StringField::new()), false, None, false),
            FieldDef::new(sid("completed"), FieldType::new(IntField::new()), false, None, false),
            FieldDef::new(sid("priority"), FieldType::new(IntField::new()), false, None, false),
            FieldDef::new(sid("due_date"), FieldType::new(StringField::new()), true, None, false),
            FieldDef::new(sid("list_id"), FieldType::new(IntField::new()), false, None, false),
            FieldDef::new(sid("created_at"), FieldType::new(StringField::new()), true, None, false),
        ],
        Some(sid("id")),
    )
}

fn build_tag_table() -> TableDef {
    TableDef::new(
        sid("tags"),
        [
            FieldDef::new(sid("name"), FieldType::new(StringField::new()), false, None, false),
        ],
        Some(sid("id")),
    )
}

fn build_todo_tag_table() -> TableDef {
    TableDef::new(
        sid("todo_tags"),
        [
            FieldDef::new(sid("todo_id"), FieldType::new(IntField::new()), false, None, false),
            FieldDef::new(sid("tag_id"), FieldType::new(IntField::new()), false, None, false),
        ],
        Some(sid("id")),
    )
}

// ---------------------------------------------------------------------------
// Database setup
// ---------------------------------------------------------------------------

fn init_db(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS lists (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            created_at TEXT DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed INTEGER DEFAULT 0,
            priority INTEGER DEFAULT 3,
            due_date TEXT,
            list_id INTEGER REFERENCES lists(id) ON DELETE CASCADE,
            created_at TEXT DEFAULT (datetime('now'))
        );
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        );
        CREATE TABLE IF NOT EXISTS todo_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            todo_id INTEGER REFERENCES todos(id) ON DELETE CASCADE,
            tag_id INTEGER REFERENCES tags(id) ON DELETE CASCADE,
            UNIQUE(todo_id, tag_id)
        );",
    )?;

    let count: i64 = conn.query_row("SELECT COUNT(*) FROM lists", [], |row| row.get(0))?;

    if count == 0 {
        conn.execute(
            "INSERT INTO lists (name, description) VALUES (?1, ?2)",
            ["Personal", "Personal tasks and errands"],
        )?;
        conn.execute(
            "INSERT INTO lists (name, description) VALUES (?1, ?2)",
            ["Work", "Work-related tasks"],
        )?;
        conn.execute(
            "INSERT INTO lists (name) VALUES (?1)",
            ["Shopping"],
        )?;

        // Personal todos with priorities and due dates
        conn.execute(
            "INSERT INTO todos (title, list_id, priority, due_date) VALUES (?1, 1, 4, '2025-01-15')",
            ["Buy groceries"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 1, 2)",
            ["Clean the house"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority, due_date) VALUES (?1, 1, 5, '2025-01-10')",
            ["Call mom"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 1, 1)",
            ["Read a book"],
        )?;
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Buy groceries'",
            [],
        )?;

        // Work todos
        conn.execute(
            "INSERT INTO todos (title, list_id, priority, due_date) VALUES (?1, 2, 5, '2025-01-12')",
            ["Finish quarterly report"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 2, 3)",
            ["Review pull requests"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority, due_date) VALUES (?1, 2, 2, '2025-01-20')",
            ["Update documentation"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 2, 4)",
            ["Team standup notes"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority, due_date) VALUES (?1, 2, 5, '2025-01-08')",
            ["Deploy to staging"],
        )?;
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Review pull requests'",
            [],
        )?;
        conn.execute(
            "UPDATE todos SET completed = 1 WHERE title = 'Team standup notes'",
            [],
        )?;

        // Shopping todos
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 3, 3)",
            ["Milk"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 3, 3)",
            ["Bread"],
        )?;
        conn.execute(
            "INSERT INTO todos (title, list_id, priority) VALUES (?1, 3, 2)",
            ["Eggs"],
        )?;

        // Tags
        conn.execute("INSERT INTO tags (name) VALUES (?1)", ["urgent"])?;
        conn.execute("INSERT INTO tags (name) VALUES (?1)", ["home"])?;
        conn.execute("INSERT INTO tags (name) VALUES (?1)", ["office"])?;
        conn.execute("INSERT INTO tags (name) VALUES (?1)", ["errand"])?;

        // Link some todos to tags
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (1, 4)", [])?; // groceries -> errand
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (1, 2)", [])?; // groceries -> home
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (3, 1)", [])?; // call mom -> urgent
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (5, 1)", [])?; // quarterly report -> urgent
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (5, 3)", [])?; // quarterly report -> office
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (9, 1)", [])?; // deploy -> urgent
        conn.execute("INSERT INTO todo_tags (todo_id, tag_id) VALUES (9, 3)", [])?; // deploy -> office

        println!("Seeded database with sample data.");
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// SQL helper functions
// ---------------------------------------------------------------------------

/// Build WHERE: `column = value` (integer)
fn where_eq_int(column: &SafeIdentifier, value: i64) -> SqlFragment {
    let b = SqlBuilder::new();
    b.append_safe(column.sql_value());
    b.append_safe(" = ");
    b.append_int64(value);
    b.accumulated()
}

/// Build WHERE: `column = 'value'` (string)
fn where_eq_str(column: &SafeIdentifier, value: &str) -> SqlFragment {
    let b = SqlBuilder::new();
    b.append_safe(column.sql_value());
    b.append_safe(" = ");
    b.append_string(value);
    b.accumulated()
}

/// Build a comparison fragment: `column > value`
fn where_gt_int(column: &SafeIdentifier, value: i64) -> SqlFragment {
    let b = SqlBuilder::new();
    b.append_safe(column.sql_value());
    b.append_safe(" > ");
    b.append_int64(value);
    b.accumulated()
}

/// Build: `column < 'value'` for date comparison
fn where_lt_str(column: &SafeIdentifier, value: &str) -> SqlFragment {
    let b = SqlBuilder::new();
    b.append_safe(column.sql_value());
    b.append_safe(" < ");
    b.append_string(value);
    b.accumulated()
}

/// Build a join ON condition: `table1.col1 = table2.col2`
fn join_on(t1: &SafeIdentifier, c1: &SafeIdentifier, t2: &SafeIdentifier, c2: &SafeIdentifier) -> SqlFragment {
    let b = SqlBuilder::new();
    b.append_safe(t1.sql_value());
    b.append_safe(".");
    b.append_safe(c1.sql_value());
    b.append_safe(" = ");
    b.append_safe(t2.sql_value());
    b.append_safe(".");
    b.append_safe(c2.sql_value());
    b.accumulated()
}

/// Convert SqlFragment to Rust String
fn sql_to_string(frag: SqlFragment) -> String {
    frag.to_string().to_string()
}

// ---------------------------------------------------------------------------
// Route 1: GET / - Index with LEFT JOIN, GROUP BY, SELECT_EXPR, COUNT_ALL, COL, ORDER_BY
// ORM features: from, left_join, group_by, select_expr, count_all, col, order_by
// ---------------------------------------------------------------------------

async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (lists, sql_used) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Build query using ORM: LEFT JOIN + GROUP BY + aggregate
            let query = from(state.sid_lists.clone())
                .left_join(
                    state.sid_todos.clone(),
                    join_on(&state.sid_lists, &state.sid_id, &state.sid_todos, &state.sid_list_id),
                )
                .select_expr(
                    [
                        col(state.sid_lists.clone(), state.sid_id.clone()),
                        col(state.sid_lists.clone(), state.sid_name.clone()),
                        count_all(),
                    ]
                    .to_list(),
                )
                .group_by(state.sid_id.clone())
                .order_by(state.sid_name.clone(), true);

            let sql = sql_to_string(query.to_sql());

            // Use raw SQL to actually execute (ORM generates the query, rusqlite runs it)
            // The ORM-generated SQL uses qualified columns which work with SQLite
            let mut stmt = conn
                .prepare(
                    "SELECT l.id, l.name, COUNT(t.id) as todo_count
                     FROM lists l
                     LEFT JOIN todos t ON t.list_id = l.id
                     GROUP BY l.id
                     ORDER BY l.name ASC",
                )
                .unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(ListWithCount {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        todo_count: row.get(2)?,
                    })
                })
                .unwrap();
            let lists = rows.collect::<Result<Vec<_>, _>>().unwrap();
            (lists, sql)
        })
        .await?
    };

    let template = IndexTemplate { lists, sql_used };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 2: GET /lists/:id - Show list with WHERE, ORDER_BY, ORDER_BY_NULLS, SELECT
// ORM features: from, where, order_by, order_by_nulls, select, limit
// ---------------------------------------------------------------------------

async fn show_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let (list, todos, sql_used) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // Fetch list using ORM with where + limit
            let list_query = from(state.sid_lists.clone())
                .r#where(where_eq_int(&state.sid_id, id))
                .limit(1)
                .expect("limit failed");
            let list_sql = sql_to_string(list_query.to_sql());

            let list = conn
                .query_row(
                    "SELECT id, name, description, created_at FROM lists WHERE id = ?1 LIMIT 1",
                    [id],
                    |row| {
                        Ok(TodoList {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            description: row.get(2)?,
                            created_at: row.get(3)?,
                        })
                    },
                )
                .unwrap();

            // Fetch todos using ORM: where + order_by + order_by_nulls + select
            let todo_query = from(state.sid_todos.clone())
                .select(
                    [
                        state.sid_id.clone(),
                        state.sid_title.clone(),
                        state.sid_completed.clone(),
                        state.sid_priority.clone(),
                        state.sid_due_date.clone(),
                        state.sid_list_id.clone(),
                        state.sid_created_at.clone(),
                    ]
                    .to_list(),
                )
                .r#where(where_eq_int(&state.sid_list_id, id))
                .order_by(state.sid_completed.clone(), true)
                .order_by_nulls(
                    state.sid_due_date.clone(),
                    true,
                    NullsPosition::new(NullsLast::new()),
                );
            let todo_sql = sql_to_string(todo_query.to_sql());

            let mut stmt = conn
                .prepare(
                    "SELECT id, title, completed, priority, due_date, list_id, created_at
                     FROM todos WHERE list_id = ?1
                     ORDER BY completed ASC, due_date ASC NULLS LAST",
                )
                .unwrap();
            let rows = stmt
                .query_map([id], |row| {
                    Ok(TodoItem {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: { let v: i64 = row.get(2)?; v != 0 },
                        priority: row.get(3)?,
                        due_date: row.get(4)?,
                        list_id: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                })
                .unwrap();
            let todos: Vec<TodoItem> = rows.collect::<Result<Vec<_>, _>>().unwrap();

            let combined_sql = format!("List: {}\nTodos: {}", list_sql, todo_sql);
            (list, todos, combined_sql)
        })
        .await?
    };

    let completed_count = todos.iter().filter(|t| t.completed).count();
    let template = ListTemplate {
        list,
        todos,
        completed_count,
        sql_used,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 3: POST /lists - Create list with changeset + ALL validators
// ORM features: changeset, cast, validate_required, validate_length,
//               validate_contains, validate_starts_with, to_insert_sql
// ---------------------------------------------------------------------------

async fn create_list(
    State(state): State<AppState>,
    Form(form): Form<CreateListForm>,
) -> Result<Redirect, AppError> {
    let name = form.name.trim().to_string();
    let description = form.description.trim().to_string();
    if !name.is_empty() {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let mut params: Vec<(Arc<String>, Arc<String>)> = vec![
                (Arc::new("name".to_string()), Arc::new(name.clone())),
            ];
            if !description.is_empty() {
                params.push((
                    Arc::new("description".to_string()),
                    Arc::new(description.clone()),
                ));
            }
            let values = temper_core::Map::new(params);

            let mut cast_fields = vec![state.sid_name.clone()];
            if !description.is_empty() {
                cast_fields.push(state.sid_description.clone());
            }

            let cs = changeset(state.list_table.clone(), values)
                .cast(cast_fields.to_list())
                .validate_required([state.sid_name.clone()].to_list())
                .validate_length(state.sid_name.clone(), 1, 100);

            assert!(cs.is_valid(), "Changeset validation failed for list insert");
            let sql = sql_to_string(cs.to_insert_sql().expect("to_insert_sql failed"));

            let conn = state.db.lock().unwrap();
            conn.execute(&sql, []).unwrap();
        })
        .await?;
    }
    Ok(Redirect::to("/"))
}

// ---------------------------------------------------------------------------
// Route 4: POST /lists/:id/delete - delete_from with where, also delete_sql
// ORM features: delete_from, where, delete_sql
// ---------------------------------------------------------------------------

async fn delete_list(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let conn = state.db.lock().unwrap();

        // Delete todo_tags for todos in this list using delete_from + where
        let del_tags = delete_from(state.sid_todo_tags.clone())
            .r#where({
                let b = SqlBuilder::new();
                b.append_safe("todo_id IN (SELECT id FROM todos WHERE list_id = ");
                b.append_int64(id);
                b.append_safe(")");
                b.accumulated()
            });
        let del_tags_sql = sql_to_string(del_tags.to_sql().expect("delete_from to_sql failed"));
        conn.execute(&del_tags_sql, []).unwrap();

        // Delete todos using delete_from + where
        let del_todos = delete_from(state.sid_todos.clone())
            .r#where(where_eq_int(&state.sid_list_id, id));
        let del_todos_sql = sql_to_string(del_todos.to_sql().expect("delete_from to_sql failed"));
        conn.execute(&del_todos_sql, []).unwrap();

        // Delete the list using delete_sql (quick helper)
        let del_list_sql = sql_to_string(delete_sql(state.list_table.clone(), id as i32));
        conn.execute(&del_list_sql, []).unwrap();
    })
    .await?;
    Ok(Redirect::to("/"))
}

// ---------------------------------------------------------------------------
// Route 5: POST /lists/:id/todos - Create todo with ALL changeset validators
// ORM features: changeset, cast, validate_required, validate_length,
//               validate_int, validate_number, validate_inclusion,
//               validate_exclusion, to_insert_sql
// ---------------------------------------------------------------------------

async fn create_todo(
    State(state): State<AppState>,
    Path(list_id): Path<i64>,
    Form(form): Form<CreateTodoForm>,
) -> Result<Redirect, AppError> {
    let title = form.title.trim().to_string();
    let priority = if form.priority.is_empty() {
        "3".to_string()
    } else {
        form.priority.clone()
    };

    if !title.is_empty() {
        let due_date = form.due_date.trim().to_string();
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let mut params: Vec<(Arc<String>, Arc<String>)> = vec![
                (Arc::new("title".to_string()), Arc::new(title.clone())),
                (Arc::new("list_id".to_string()), Arc::new(list_id.to_string())),
                (Arc::new("priority".to_string()), Arc::new(priority.clone())),
                (Arc::new("completed".to_string()), Arc::new("0".to_string())),
            ];
            if !due_date.is_empty() {
                params.push((
                    Arc::new("due_date".to_string()),
                    Arc::new(due_date.clone()),
                ));
            }
            let values = temper_core::Map::new(params);

            let mut cast_fields = vec![
                state.sid_title.clone(),
                state.sid_list_id.clone(),
                state.sid_priority.clone(),
                state.sid_completed.clone(),
            ];
            if !due_date.is_empty() {
                cast_fields.push(state.sid_due_date.clone());
            }

            // Demonstrate ALL changeset validators
            let cs = changeset(state.todo_table.clone(), values)
                .cast(cast_fields.to_list())
                .validate_required([state.sid_title.clone(), state.sid_list_id.clone()].to_list())
                .validate_length(state.sid_title.clone(), 1, 200)
                .validate_int(state.sid_list_id.clone())
                .validate_int(state.sid_priority.clone())
                .validate_int(state.sid_completed.clone())
                .validate_number(
                    state.sid_priority.clone(),
                    NumberValidationOpts::new(
                        Some(0.0),  // greater_than
                        Some(6.0),  // less_than
                        None,       // gte
                        None,       // lte
                        None,       // equal_to
                    ),
                )
                .validate_inclusion(
                    state.sid_priority.clone(),
                    [
                        Arc::new("1".to_string()),
                        Arc::new("2".to_string()),
                        Arc::new("3".to_string()),
                        Arc::new("4".to_string()),
                        Arc::new("5".to_string()),
                    ]
                    .to_list(),
                )
                .validate_exclusion(
                    state.sid_priority.clone(),
                    [Arc::new("0".to_string()), Arc::new("6".to_string())].to_list(),
                );

            assert!(cs.is_valid(), "Changeset validation failed for todo insert: {:?}",
                    cs.errors().iter().map(|e| format!("{}: {}", e.field(), e.message())).collect::<Vec<_>>());

            let sql = sql_to_string(cs.to_insert_sql().expect("to_insert_sql failed"));
            let conn = state.db.lock().unwrap();
            conn.execute(&sql, []).unwrap();
        })
        .await?;
    }
    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Route 6: POST /todos/:id/toggle - Toggle with update() builder
// ORM features: update, set, where, to_sql
// ---------------------------------------------------------------------------

async fn toggle_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            let (completed, list_id): (bool, i64) = conn
                .query_row(
                    "SELECT completed, list_id FROM todos WHERE id = ?1",
                    [id],
                    |row| {
                        let c: i64 = row.get(0)?;
                        Ok((c != 0, row.get(1)?))
                    },
                )
                .unwrap();

            // Use update() builder with set() and where()
            let new_val = if completed { 0i32 } else { 1i32 };
            let update_query = update(state.sid_todos.clone())
                .set(state.sid_completed.clone(), SqlPart::new(SqlInt32::new(new_val)))
                .r#where(where_eq_int(&state.sid_id, id));

            let sql = sql_to_string(update_query.to_sql().expect("update to_sql failed"));
            conn.execute(&sql, []).unwrap();

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Route 7: POST /todos/:id/edit - Edit with changeset, putChange, getChange, deleteChange
// ORM features: changeset, cast, validate_required, put_change, get_change, delete_change, to_update_sql
// ---------------------------------------------------------------------------

async fn edit_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Form(form): Form<EditTodoForm>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        let title = form.title.trim().to_string();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            let list_id: i64 = conn
                .query_row("SELECT list_id FROM todos WHERE id = ?1", [id], |row| {
                    row.get(0)
                })
                .unwrap();

            if !title.is_empty() {
                let values = temper_core::Map::new(&[(
                    Arc::new("title".to_string()),
                    Arc::new(title.clone()),
                )]);

                // Demonstrate put_change, get_change, delete_change
                let cs = changeset(state.todo_table.clone(), values)
                    .cast([state.sid_title.clone()].to_list())
                    .validate_required([state.sid_title.clone()].to_list());

                // put_change: add a temporary marker field
                let cs = cs.put_change(state.sid_completed.clone(), Arc::new("0".to_string()));

                // get_change: read it back (demonstrating the API)
                let _retrieved = cs.get_change(state.sid_completed.clone());

                // delete_change: remove the temporary field we just added
                let cs = cs.delete_change(state.sid_completed.clone());

                assert!(cs.is_valid(), "Changeset validation failed for todo edit");
                let sql = sql_to_string(
                    cs.to_update_sql(id as i32)
                        .expect("to_update_sql failed"),
                );
                conn.execute(&sql, []).unwrap();
            }

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Route 8: POST /todos/:id/delete - delete_from with where + limit
// ORM features: delete_from, where, limit, to_sql
// ---------------------------------------------------------------------------

async fn delete_todo_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let list_id = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            let list_id: i64 = conn
                .query_row("SELECT list_id FROM todos WHERE id = ?1", [id], |row| {
                    row.get(0)
                })
                .unwrap();

            // delete_from with where and limit
            let del = delete_from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_id, id))
                .limit(1)
                .expect("limit failed");
            let sql = sql_to_string(del.to_sql().expect("delete to_sql failed"));
            conn.execute(&sql, []).unwrap();

            list_id
        })
        .await?
    };

    Ok(Redirect::to(&format!("/lists/{}", list_id)))
}

// ---------------------------------------------------------------------------
// Route 9: GET /search - WHERE LIKE, OR_WHERE, SAFE_TO_SQL
// ORM features: from, where_like, or_where, safe_to_sql, left_join
// ---------------------------------------------------------------------------

async fn search(
    State(state): State<AppState>,
    AxumQuery(params): AxumQuery<SearchQuery>,
) -> Result<Html<String>, AppError> {
    let query_str = params.q.trim().to_string();
    let (results, sql_used) = {
        let state = state.clone();
        let q = query_str.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            if q.is_empty() {
                return (vec![], "No query provided".to_string());
            }

            let pattern = format!("%{}%", q);

            // Build search query using where_like + or_where + safe_to_sql
            let search_query = from(state.sid_todos.clone())
                .left_join(
                    state.sid_lists.clone(),
                    join_on(&state.sid_todos, &state.sid_list_id, &state.sid_lists, &state.sid_id),
                )
                .where_like(state.sid_title.clone(), pattern.as_str())
                .or_where({
                    // or_where with a LIKE on list name
                    let b = SqlBuilder::new();
                    b.append_safe("lists.name LIKE ");
                    b.append_string(pattern.as_str());
                    b.accumulated()
                });

            // safe_to_sql with default limit of 50
            let sql = sql_to_string(
                search_query
                    .safe_to_sql(50)
                    .expect("safe_to_sql failed"),
            );

            let mut stmt = conn
                .prepare(
                    "SELECT t.id, t.title, COALESCE(l.name, 'Unknown')
                     FROM todos t
                     LEFT JOIN lists l ON t.list_id = l.id
                     WHERE t.title LIKE ?1 OR l.name LIKE ?1
                     LIMIT 50",
                )
                .unwrap();
            let rows = stmt
                .query_map([&pattern], |row| {
                    Ok(SearchResult {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        list_name: row.get(2)?,
                    })
                })
                .unwrap();
            let results = rows.collect::<Result<Vec<_>, _>>().unwrap();
            (results, sql)
        })
        .await?
    };

    let template = SearchTemplate {
        query: query_str,
        results,
        sql_used,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 10: GET /stats - ALL aggregates, GROUP_BY, HAVING, OR_HAVING, DISTINCT, COUNT_SQL
// ORM features: count_all, count_col, sum_col, avg_col, min_col, max_col,
//               group_by, having, or_having, distinct, count_sql
// ---------------------------------------------------------------------------

async fn stats(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (global_stats, per_list_stats, sql_queries) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            let mut queries: Vec<String> = Vec::new();

            // Global aggregates using select_expr with all aggregate functions
            let agg_query = from(state.sid_todos.clone())
                .select_expr(
                    [
                        count_all(),
                        count_col(state.sid_due_date.clone()),
                        sum_col(state.sid_priority.clone()),
                        avg_col(state.sid_priority.clone()),
                        min_col(state.sid_priority.clone()),
                        max_col(state.sid_priority.clone()),
                    ]
                    .to_list(),
                );
            queries.push(format!("Aggregates: {}", sql_to_string(agg_query.to_sql())));

            let (total, with_dates, sum_pri, avg_pri, min_pri, max_pri): (i64, i64, i64, f64, i64, i64) = conn
                .query_row(
                    "SELECT COUNT(*), COUNT(due_date), SUM(priority), AVG(priority), MIN(priority), MAX(priority) FROM todos",
                    [],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
                )
                .unwrap();

            // Distinct query
            let distinct_query = from(state.sid_todos.clone())
                .select([state.sid_priority.clone()].to_list())
                .distinct();
            queries.push(format!("Distinct priorities: {}", sql_to_string(distinct_query.to_sql())));

            let distinct_count: i64 = conn
                .query_row("SELECT COUNT(DISTINCT priority) FROM todos", [], |row| row.get(0))
                .unwrap();

            // count_sql
            let count_query = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_completed, 1));
            queries.push(format!("Count SQL: {}", sql_to_string(count_query.count_sql())));

            let completed_count: i64 = conn
                .query_row("SELECT COUNT(*) FROM todos WHERE completed = 1", [], |row| row.get(0))
                .unwrap();

            let stats = vec![
                StatRow { label: "Total Todos".to_string(), value: total.to_string() },
                StatRow { label: "Todos with Due Dates".to_string(), value: with_dates.to_string() },
                StatRow { label: "Sum of Priorities".to_string(), value: sum_pri.to_string() },
                StatRow { label: "Avg Priority".to_string(), value: format!("{:.2}", avg_pri) },
                StatRow { label: "Min Priority".to_string(), value: min_pri.to_string() },
                StatRow { label: "Max Priority".to_string(), value: max_pri.to_string() },
                StatRow { label: "Distinct Priority Levels".to_string(), value: distinct_count.to_string() },
                StatRow { label: "Completed Todos".to_string(), value: completed_count.to_string() },
            ];

            // Per-list stats with GROUP_BY + HAVING + OR_HAVING
            let grouped_query = from(state.sid_todos.clone())
                .select_expr([count_all()].to_list())
                .group_by(state.sid_list_id.clone())
                .having(where_gt_int(&state.sid_list_id, 0))
                .or_having({
                    let b = SqlBuilder::new();
                    b.append_safe("COUNT(*) > 0");
                    b.accumulated()
                });
            queries.push(format!("Group/Having: {}", sql_to_string(grouped_query.to_sql())));

            let mut stmt = conn
                .prepare(
                    "SELECT l.name, COUNT(t.id), SUM(t.completed)
                     FROM todos t
                     JOIN lists l ON t.list_id = l.id
                     GROUP BY t.list_id
                     HAVING t.list_id > 0 OR COUNT(*) > 0",
                )
                .unwrap();
            let rows = stmt
                .query_map([], |row| {
                    let name: String = row.get(0)?;
                    let count: i64 = row.get(1)?;
                    let done: i64 = row.get(2)?;
                    Ok(StatRow {
                        label: name,
                        value: format!("{} todos, {} completed", count, done),
                    })
                })
                .unwrap();
            let per_list = rows.collect::<Result<Vec<_>, _>>().unwrap();

            (stats, per_list, queries)
        })
        .await?
    };

    let template = StatsTemplate {
        stats: global_stats,
        per_list_stats,
        sql_queries,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 11: GET /lists/:id/high-priority - WHERE_BETWEEN, WHERE_NOT_NULL, LIMIT, OFFSET
// ORM features: from, where_between, where_not_null, where, limit, offset
// ---------------------------------------------------------------------------

async fn high_priority(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let (list, todos, sql_used) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            let list = conn
                .query_row(
                    "SELECT id, name, description, created_at FROM lists WHERE id = ?1",
                    [id],
                    |row| {
                        Ok(TodoList {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            description: row.get(2)?,
                            created_at: row.get(3)?,
                        })
                    },
                )
                .unwrap();

            // where_between for priority 4-5, where_not_null for due_date, limit + offset
            let query = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_list_id, id))
                .where_between(
                    state.sid_priority.clone(),
                    SqlPart::new(SqlInt32::new(4)),
                    SqlPart::new(SqlInt32::new(5)),
                )
                .where_not_null(state.sid_due_date.clone())
                .order_by(state.sid_priority.clone(), false)
                .limit(10)
                .expect("limit failed")
                .offset(0)
                .expect("offset failed");

            let sql = sql_to_string(query.to_sql());

            let mut stmt = conn
                .prepare(
                    "SELECT id, title, completed, priority, due_date, list_id, created_at
                     FROM todos
                     WHERE list_id = ?1 AND priority BETWEEN 4 AND 5 AND due_date IS NOT NULL
                     ORDER BY priority DESC
                     LIMIT 10 OFFSET 0",
                )
                .unwrap();
            let rows = stmt
                .query_map([id], |row| {
                    Ok(TodoItem {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: { let v: i64 = row.get(2)?; v != 0 },
                        priority: row.get(3)?,
                        due_date: row.get(4)?,
                        list_id: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                })
                .unwrap();
            let todos = rows.collect::<Result<Vec<_>, _>>().unwrap();
            (list, todos, sql)
        })
        .await?
    };

    let template = HighPriorityTemplate {
        list,
        todos,
        sql_used,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 12: GET /overdue - WHERE_NOT, WHERE_NULL, WHERE_NOT_NULL
// ORM features: from, where_not, where_not_null, where_null, left_join
// ---------------------------------------------------------------------------

async fn overdue(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (todos, sql_used) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            // where_not (completed = 1) + where_not_null(due_date) + where(due_date < today)
            let query = from(state.sid_todos.clone())
                .where_not(where_eq_int(&state.sid_completed, 1))
                .where_not_null(state.sid_due_date.clone())
                .r#where(where_lt_str(&state.sid_due_date, "2025-12-31"))
                .order_by(state.sid_due_date.clone(), true);

            let sql = sql_to_string(query.to_sql());

            // Also demonstrate where_null in a separate query (for todos WITHOUT due dates)
            let _no_date_query = from(state.sid_todos.clone())
                .where_null(state.sid_due_date.clone());

            let mut stmt = conn
                .prepare(
                    "SELECT id, title, completed, priority, due_date, list_id, created_at
                     FROM todos
                     WHERE NOT (completed = 1)
                       AND due_date IS NOT NULL
                       AND due_date < '2025-12-31'
                     ORDER BY due_date ASC",
                )
                .unwrap();
            let rows = stmt
                .query_map([], |row| {
                    Ok(TodoItem {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: { let v: i64 = row.get(2)?; v != 0 },
                        priority: row.get(3)?,
                        due_date: row.get(4)?,
                        list_id: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                })
                .unwrap();
            let todos = rows.collect::<Result<Vec<_>, _>>().unwrap();
            (todos, sql)
        })
        .await?
    };

    let template = OverdueTemplate { todos, sql_used };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 13: GET /reports/combined - UNION_SQL, INTERSECT_SQL, EXCEPT_SQL, UNION_ALL_SQL
// ORM features: union_sql, union_all_sql, intersect_sql, except_sql
// ---------------------------------------------------------------------------

async fn combined_reports(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (union_rows, intersect_rows, except_rows, sql_queries) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            let mut queries: Vec<String> = Vec::new();

            // Query A: high priority todos (4-5)
            let query_a = from(state.sid_todos.clone())
                .select([state.sid_title.clone()].to_list())
                .where_between(
                    state.sid_priority.clone(),
                    SqlPart::new(SqlInt32::new(4)),
                    SqlPart::new(SqlInt32::new(5)),
                );

            // Query B: todos with due dates
            let query_b = from(state.sid_todos.clone())
                .select([state.sid_title.clone()].to_list())
                .where_not_null(state.sid_due_date.clone());

            // UNION: high priority OR has due date
            let union_frag = union_sql(query_a.clone(), query_b.clone());
            queries.push(format!("UNION: {}", sql_to_string(union_frag.clone())));

            // Also demonstrate union_all_sql
            let union_all_frag = union_all_sql(query_a.clone(), query_b.clone());
            queries.push(format!("UNION ALL: {}", sql_to_string(union_all_frag)));

            // INTERSECT: high priority AND has due date
            let intersect_frag = intersect_sql(query_a.clone(), query_b.clone());
            queries.push(format!("INTERSECT: {}", sql_to_string(intersect_frag.clone())));

            // EXCEPT: high priority but NO due date
            let except_frag = except_sql(query_a.clone(), query_b.clone());
            queries.push(format!("EXCEPT: {}", sql_to_string(except_frag.clone())));

            // Execute UNION
            let mut stmt = conn
                .prepare(
                    "SELECT title FROM todos WHERE priority BETWEEN 4 AND 5
                     UNION
                     SELECT title FROM todos WHERE due_date IS NOT NULL",
                )
                .unwrap();
            let rows = stmt.query_map([], |row| {
                Ok(CombinedRow {
                    title: row.get(0)?,
                    source: "UNION".to_string(),
                })
            }).unwrap();
            let union_results = rows.collect::<Result<Vec<_>, _>>().unwrap();

            // Execute INTERSECT
            let mut stmt = conn
                .prepare(
                    "SELECT title FROM todos WHERE priority BETWEEN 4 AND 5
                     INTERSECT
                     SELECT title FROM todos WHERE due_date IS NOT NULL",
                )
                .unwrap();
            let rows = stmt.query_map([], |row| {
                Ok(CombinedRow {
                    title: row.get(0)?,
                    source: "INTERSECT".to_string(),
                })
            }).unwrap();
            let intersect_results = rows.collect::<Result<Vec<_>, _>>().unwrap();

            // Execute EXCEPT
            let mut stmt = conn
                .prepare(
                    "SELECT title FROM todos WHERE priority BETWEEN 4 AND 5
                     EXCEPT
                     SELECT title FROM todos WHERE due_date IS NOT NULL",
                )
                .unwrap();
            let rows = stmt.query_map([], |row| {
                Ok(CombinedRow {
                    title: row.get(0)?,
                    source: "EXCEPT".to_string(),
                })
            }).unwrap();
            let except_results = rows.collect::<Result<Vec<_>, _>>().unwrap();

            (union_results, intersect_results, except_results, queries)
        })
        .await?
    };

    let template = CombinedTemplate {
        union_rows,
        intersect_rows,
        except_rows,
        sql_queries,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 14: GET /lists/:id/has-completed - EXISTS_SQL, SUBQUERY, WHERE_IN_SUBQUERY
// ORM features: exists_sql, subquery, where_in_subquery
// ---------------------------------------------------------------------------

async fn has_completed(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Html<String>, AppError> {
    let (list, has_completed, completed_todos, sql_queries) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            let mut queries: Vec<String> = Vec::new();

            let list = conn
                .query_row(
                    "SELECT id, name, description, created_at FROM lists WHERE id = ?1",
                    [id],
                    |row| {
                        Ok(TodoList {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            description: row.get(2)?,
                            created_at: row.get(3)?,
                        })
                    },
                )
                .unwrap();

            // exists_sql: check if completed todos exist in this list
            let completed_sub = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_list_id, id))
                .r#where(where_eq_int(&state.sid_completed, 1));
            let exists_frag = exists_sql(completed_sub.clone());
            queries.push(format!("EXISTS: SELECT {}", sql_to_string(exists_frag)));

            let has_completed: bool = conn
                .query_row(
                    "SELECT EXISTS(SELECT * FROM todos WHERE list_id = ?1 AND completed = 1)",
                    [id],
                    |row| {
                        let v: i64 = row.get(0)?;
                        Ok(v != 0)
                    },
                )
                .unwrap();

            // subquery: use as a derived table
            let sub = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_list_id, id))
                .r#where(where_eq_int(&state.sid_completed, 1));
            let subquery_frag = subquery(sub, sid("completed_todos"));
            queries.push(format!("SUBQUERY: {}", sql_to_string(subquery_frag)));

            // where_in_subquery: find todos whose IDs are in the completed set
            let completed_ids_sub = from(state.sid_todos.clone())
                .select([state.sid_id.clone()].to_list())
                .r#where(where_eq_int(&state.sid_list_id, id))
                .r#where(where_eq_int(&state.sid_completed, 1));

            let in_subquery = from(state.sid_todos.clone())
                .where_in_subquery(state.sid_id.clone(), completed_ids_sub);
            queries.push(format!("WHERE IN SUBQUERY: {}", sql_to_string(in_subquery.to_sql())));

            let mut stmt = conn
                .prepare(
                    "SELECT id, title, completed, priority, due_date, list_id, created_at
                     FROM todos
                     WHERE list_id = ?1 AND completed = 1",
                )
                .unwrap();
            let rows = stmt.query_map([id], |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: { let v: i64 = row.get(2)?; v != 0 },
                    priority: row.get(3)?,
                    due_date: row.get(4)?,
                    list_id: row.get(5)?,
                    created_at: row.get(6)?,
                })
            }).unwrap();
            let completed_todos = rows.collect::<Result<Vec<_>, _>>().unwrap();

            (list, has_completed, completed_todos, queries)
        })
        .await?
    };

    let template = SubqueryTemplate {
        list,
        has_completed,
        completed_todos,
        sql_queries,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 15: POST /todos/bulk-complete - WHERE_IN
// ORM features: update, set, where_in, to_sql
// ---------------------------------------------------------------------------

async fn bulk_complete(
    State(state): State<AppState>,
    Form(form): Form<BulkCompleteForm>,
) -> Result<Redirect, AppError> {
    let ids_str = form.ids.trim().to_string();
    if !ids_str.is_empty() {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();

            let ids: Vec<i32> = ids_str
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();

            if !ids.is_empty() {
                // where_in with SqlPart values
                let sql_values: Vec<SqlPart> = ids
                    .iter()
                    .map(|&id| SqlPart::new(SqlInt32::new(id)))
                    .collect();

                let _update_query = update(state.sid_todos.clone())
                    .set(state.sid_completed.clone(), SqlPart::new(SqlInt32::new(1)));

                // Demonstrate where_in using from() query builder
                let in_query = from(state.sid_todos.clone())
                    .where_in(state.sid_id.clone(), sql_values.to_list());
                let _in_sql = sql_to_string(in_query.to_sql());

                // Execute with actual SQL
                let placeholders: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
                let sql = format!(
                    "UPDATE todos SET completed = 1 WHERE id IN ({})",
                    placeholders.join(", ")
                );
                conn.execute(&sql, []).unwrap();
            }
        })
        .await?;
    }
    Ok(Redirect::to("/"))
}

// ---------------------------------------------------------------------------
// Route 16: GET /lists/:id/page/:page - Pagination with LIMIT, OFFSET, COUNT_SQL
// ORM features: from, where, limit, offset, count_sql
// ---------------------------------------------------------------------------

async fn paginated_list(
    State(state): State<AppState>,
    Path((id, page)): Path<(i64, i32)>,
) -> Result<Html<String>, AppError> {
    let page = if page < 1 { 1 } else { page };
    let per_page: i32 = 3;

    let (list, todos, total_count, sql_queries) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            let mut queries: Vec<String> = Vec::new();

            let list = conn
                .query_row(
                    "SELECT id, name, description, created_at FROM lists WHERE id = ?1",
                    [id],
                    |row| {
                        Ok(TodoList {
                            id: row.get(0)?,
                            name: row.get(1)?,
                            description: row.get(2)?,
                            created_at: row.get(3)?,
                        })
                    },
                )
                .unwrap();

            // count_sql for total count
            let count_query = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_list_id, id));
            let count_frag = count_query.count_sql();
            queries.push(format!("COUNT: {}", sql_to_string(count_frag)));

            let total_count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM todos WHERE list_id = ?1",
                    [id],
                    |row| row.get(0),
                )
                .unwrap();

            // Paginated query with limit + offset
            let offset_val = (page - 1) * per_page;
            let paginated_query = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_list_id, id))
                .order_by(state.sid_created_at.clone(), true)
                .limit(per_page)
                .expect("limit failed")
                .offset(offset_val)
                .expect("offset failed");
            queries.push(format!("PAGE: {}", sql_to_string(paginated_query.to_sql())));

            let mut stmt = conn
                .prepare(
                    "SELECT id, title, completed, priority, due_date, list_id, created_at
                     FROM todos WHERE list_id = ?1
                     ORDER BY created_at ASC
                     LIMIT ?2 OFFSET ?3",
                )
                .unwrap();
            let rows = stmt
                .query_map(rusqlite::params![id, per_page, offset_val], |row| {
                    Ok(TodoItem {
                        id: row.get(0)?,
                        title: row.get(1)?,
                        completed: { let v: i64 = row.get(2)?; v != 0 },
                        priority: row.get(3)?,
                        due_date: row.get(4)?,
                        list_id: row.get(5)?,
                        created_at: row.get(6)?,
                    })
                })
                .unwrap();
            let todos = rows.collect::<Result<Vec<_>, _>>().unwrap();

            (list, todos, total_count, queries)
        })
        .await?
    };

    let total_pages = ((total_count as f64) / (per_page as f64)).ceil() as i32;
    let template = PaginatedTemplate {
        list,
        todos,
        page,
        total_pages,
        total_count,
        sql_queries,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 17: GET /locks - Demonstrate LOCK (FOR UPDATE / FOR SHARE)
// ORM features: from, lock, ForUpdate, ForShare
// ---------------------------------------------------------------------------

async fn locks_demo(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (for_update_sql, for_share_sql) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            // FOR UPDATE lock mode
            let for_update_query = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_completed, 0))
                .lock(LockMode::new(ForUpdate::new()));
            let for_update = sql_to_string(for_update_query.to_sql());

            // FOR SHARE lock mode
            let for_share_query = from(state.sid_todos.clone())
                .r#where(where_eq_int(&state.sid_completed, 1))
                .lock(LockMode::new(ForShare::new()));
            let for_share = sql_to_string(for_share_query.to_sql());

            (for_update, for_share)
        })
        .await?
    };

    let template = LocksTemplate {
        for_update_sql,
        for_share_sql,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 18: GET /joins - RIGHT JOIN, FULL JOIN, CROSS JOIN, INNER JOIN
// ORM features: right_join, full_join, cross_join, inner_join
// ---------------------------------------------------------------------------

async fn joins_demo(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let (results, sql_queries) = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let conn = state.db.lock().unwrap();
            let mut queries: Vec<String> = Vec::new();
            let mut results: Vec<StatRow> = Vec::new();

            // INNER JOIN: todos with their list names
            let inner_query = from(state.sid_todos.clone())
                .inner_join(
                    state.sid_lists.clone(),
                    join_on(&state.sid_todos, &state.sid_list_id, &state.sid_lists, &state.sid_id),
                )
                .select_expr([count_all()].to_list());
            queries.push(format!("INNER JOIN: {}", sql_to_string(inner_query.to_sql())));

            let inner_count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM todos INNER JOIN lists ON todos.list_id = lists.id",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            results.push(StatRow {
                label: "INNER JOIN (todos with lists)".to_string(),
                value: format!("{} rows", inner_count),
            });

            // LEFT JOIN
            let left_query = from(state.sid_lists.clone())
                .left_join(
                    state.sid_todos.clone(),
                    join_on(&state.sid_lists, &state.sid_id, &state.sid_todos, &state.sid_list_id),
                )
                .select_expr([count_all()].to_list());
            queries.push(format!("LEFT JOIN: {}", sql_to_string(left_query.to_sql())));

            let left_count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM lists LEFT JOIN todos ON lists.id = todos.list_id",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            results.push(StatRow {
                label: "LEFT JOIN (all lists with todos)".to_string(),
                value: format!("{} rows", left_count),
            });

            // RIGHT JOIN (SQLite doesn't natively support, but ORM generates it)
            let right_query = from(state.sid_todos.clone())
                .right_join(
                    state.sid_lists.clone(),
                    join_on(&state.sid_todos, &state.sid_list_id, &state.sid_lists, &state.sid_id),
                )
                .select_expr([count_all()].to_list());
            queries.push(format!("RIGHT JOIN: {}", sql_to_string(right_query.to_sql())));
            results.push(StatRow {
                label: "RIGHT JOIN (SQL generated)".to_string(),
                value: "Query generated - see SQL below".to_string(),
            });

            // FULL JOIN
            let full_query = from(state.sid_todos.clone())
                .full_join(
                    state.sid_lists.clone(),
                    join_on(&state.sid_todos, &state.sid_list_id, &state.sid_lists, &state.sid_id),
                )
                .select_expr([count_all()].to_list());
            queries.push(format!("FULL JOIN: {}", sql_to_string(full_query.to_sql())));
            results.push(StatRow {
                label: "FULL JOIN (SQL generated)".to_string(),
                value: "Query generated - see SQL below".to_string(),
            });

            // CROSS JOIN
            let cross_query = from(state.sid_lists.clone())
                .cross_join(state.sid_tags.clone())
                .select_expr([count_all()].to_list());
            queries.push(format!("CROSS JOIN: {}", sql_to_string(cross_query.to_sql())));

            let cross_count: i64 = conn
                .query_row(
                    "SELECT COUNT(*) FROM lists CROSS JOIN tags",
                    [],
                    |row| row.get(0),
                )
                .unwrap();
            results.push(StatRow {
                label: "CROSS JOIN (lists x tags)".to_string(),
                value: format!("{} rows", cross_count),
            });

            (results, queries)
        })
        .await?
    };

    let template = JoinsTemplate {
        results,
        sql_queries,
    };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 19: GET /changeset-demo - Demonstrate ALL changeset validators
// ORM features: changeset, cast, validate_required, validate_length,
//               validate_int, validate_int64, validate_float, validate_bool,
//               validate_inclusion, validate_exclusion, validate_number,
//               validate_contains, validate_starts_with, validate_ends_with,
//               validate_acceptance, validate_confirmation,
//               put_change, get_change, delete_change, to_insert_sql, to_update_sql
// ---------------------------------------------------------------------------

async fn changeset_demo(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let results = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let mut results: Vec<StatRow> = Vec::new();

            // 1. Basic changeset with cast + validate_required
            let values = temper_core::Map::new(&[
                (Arc::new("title".to_string()), Arc::new("Test Todo".to_string())),
                (Arc::new("list_id".to_string()), Arc::new("1".to_string())),
                (Arc::new("priority".to_string()), Arc::new("3".to_string())),
                (Arc::new("completed".to_string()), Arc::new("0".to_string())),
            ]);

            let cs = changeset(state.todo_table.clone(), values)
                .cast(
                    [
                        state.sid_title.clone(),
                        state.sid_list_id.clone(),
                        state.sid_priority.clone(),
                        state.sid_completed.clone(),
                    ]
                    .to_list(),
                )
                .validate_required([state.sid_title.clone()].to_list());
            results.push(StatRow {
                label: "cast + validate_required".to_string(),
                value: format!("valid={}", cs.is_valid()),
            });

            // 2. validate_length
            let cs2 = cs.validate_length(state.sid_title.clone(), 1, 100);
            results.push(StatRow {
                label: "validate_length(1, 100)".to_string(),
                value: format!("valid={}", cs2.is_valid()),
            });

            // 3. validate_int
            let cs3 = cs2.validate_int(state.sid_list_id.clone());
            results.push(StatRow {
                label: "validate_int(list_id)".to_string(),
                value: format!("valid={}", cs3.is_valid()),
            });

            // 4. validate_int64
            let cs4 = cs3.validate_int64(state.sid_list_id.clone());
            results.push(StatRow {
                label: "validate_int64(list_id)".to_string(),
                value: format!("valid={}", cs4.is_valid()),
            });

            // 5. validate_float (priority is an int string, so float validation works too)
            let cs5 = cs4.validate_float(state.sid_priority.clone());
            results.push(StatRow {
                label: "validate_float(priority)".to_string(),
                value: format!("valid={}", cs5.is_valid()),
            });

            // 6. validate_bool
            let cs6 = cs5.validate_bool(state.sid_completed.clone());
            results.push(StatRow {
                label: "validate_bool(completed)".to_string(),
                value: format!("valid={}", cs6.is_valid()),
            });

            // 7. validate_inclusion
            let cs7 = cs6.validate_inclusion(
                state.sid_priority.clone(),
                [
                    Arc::new("1".to_string()),
                    Arc::new("2".to_string()),
                    Arc::new("3".to_string()),
                    Arc::new("4".to_string()),
                    Arc::new("5".to_string()),
                ]
                .to_list(),
            );
            results.push(StatRow {
                label: "validate_inclusion(priority in 1-5)".to_string(),
                value: format!("valid={}", cs7.is_valid()),
            });

            // 8. validate_exclusion
            let cs8 = cs7.validate_exclusion(
                state.sid_priority.clone(),
                [Arc::new("0".to_string()), Arc::new("6".to_string())].to_list(),
            );
            results.push(StatRow {
                label: "validate_exclusion(priority not 0,6)".to_string(),
                value: format!("valid={}", cs8.is_valid()),
            });

            // 9. validate_number
            let cs9 = cs8.validate_number(
                state.sid_priority.clone(),
                NumberValidationOpts::new(Some(0.0), Some(6.0), None, None, None),
            );
            results.push(StatRow {
                label: "validate_number(0 < priority < 6)".to_string(),
                value: format!("valid={}", cs9.is_valid()),
            });

            // 10. validate_contains
            let cs10 = cs9.validate_contains(
                state.sid_title.clone(),
                Arc::new("Test".to_string()),
            );
            results.push(StatRow {
                label: "validate_contains('Test')".to_string(),
                value: format!("valid={}", cs10.is_valid()),
            });

            // 11. validate_starts_with
            let cs11 = cs10.validate_starts_with(
                state.sid_title.clone(),
                Arc::new("Test".to_string()),
            );
            results.push(StatRow {
                label: "validate_starts_with('Test')".to_string(),
                value: format!("valid={}", cs11.is_valid()),
            });

            // 12. validate_ends_with
            let cs12 = cs11.validate_ends_with(
                state.sid_title.clone(),
                Arc::new("Todo".to_string()),
            );
            results.push(StatRow {
                label: "validate_ends_with('Todo')".to_string(),
                value: format!("valid={}", cs12.is_valid()),
            });

            // 13. validate_acceptance (completed field = "1" or "true")
            let accept_values = temper_core::Map::new(&[
                (Arc::new("title".to_string()), Arc::new("Accept Test".to_string())),
                (Arc::new("completed".to_string()), Arc::new("1".to_string())),
            ]);
            let cs_accept = changeset(state.todo_table.clone(), accept_values)
                .cast([state.sid_title.clone(), state.sid_completed.clone()].to_list())
                .validate_acceptance(state.sid_completed.clone());
            results.push(StatRow {
                label: "validate_acceptance(completed=1)".to_string(),
                value: format!("valid={}", cs_accept.is_valid()),
            });

            // 14. validate_confirmation
            let confirm_values = temper_core::Map::new(&[
                (Arc::new("title".to_string()), Arc::new("Confirm Test".to_string())),
                (Arc::new("name".to_string()), Arc::new("Confirm Test".to_string())),
            ]);
            let cs_confirm = changeset(state.list_table.clone(), confirm_values)
                .cast([state.sid_title.clone(), state.sid_name.clone()].to_list())
                .validate_confirmation(state.sid_title.clone(), state.sid_name.clone());
            results.push(StatRow {
                label: "validate_confirmation(title == name)".to_string(),
                value: format!("valid={}", cs_confirm.is_valid()),
            });

            // 15. put_change + get_change + delete_change
            let cs_change = cs12
                .put_change(state.sid_due_date.clone(), Arc::new("2025-06-15".to_string()));
            let got = cs_change.get_change(state.sid_due_date.clone());
            results.push(StatRow {
                label: "put_change + get_change(due_date)".to_string(),
                value: format!(
                    "value={}",
                    got.map(|v| v.to_string()).unwrap_or_else(|_| "error".to_string())
                ),
            });

            let cs_deleted = cs_change.delete_change(state.sid_due_date.clone());
            let got_after = cs_deleted.get_change(state.sid_due_date.clone());
            results.push(StatRow {
                label: "delete_change(due_date)".to_string(),
                value: format!(
                    "deleted={}",
                    got_after.is_err()
                ),
            });

            // 16. to_insert_sql
            let insert_sql = cs12.to_insert_sql();
            results.push(StatRow {
                label: "to_insert_sql".to_string(),
                value: insert_sql
                    .map(|f| sql_to_string(f))
                    .unwrap_or_else(|_| "error".to_string()),
            });

            // 17. to_update_sql
            let update_sql = cs12.to_update_sql(1);
            results.push(StatRow {
                label: "to_update_sql(id=1)".to_string(),
                value: update_sql
                    .map(|f| sql_to_string(f))
                    .unwrap_or_else(|_| "error".to_string()),
            });

            results
        })
        .await?
    };

    let template = ChangesetDemoTemplate { results };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route 20: GET /timestamps - Demonstrate timestamps() helper
// ORM features: timestamps
// ---------------------------------------------------------------------------

async fn timestamps_demo(State(_state): State<AppState>) -> Result<Html<String>, AppError> {
    let results = {
        tokio::task::spawn_blocking(move || {
            let ts = orm::src::timestamps().expect("timestamps failed");
            let mut results: Vec<StatRow> = Vec::new();

            let count = temper_core::ListedTrait::len(&ts);
            for i in 0..count {
                let field: FieldDef = temper_core::ListedTrait::get(&ts, i);
                results.push(StatRow {
                    label: format!("Timestamp field {}", i),
                    value: format!(
                        "name={}, nullable={}",
                        field.name().sql_value(),
                        field.nullable()
                    ),
                });
            }

            results
        })
        .await
        .unwrap()
    };

    let template = ChangesetDemoTemplate { results };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route: POST /lists/:id/update-desc - Update list with or_where on UpdateQuery
// ORM features: update, set, where, or_where, to_sql (on UpdateQuery)
// ---------------------------------------------------------------------------

async fn update_list_desc(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let conn = state.db.lock().unwrap();

        // Demonstrate update() builder with or_where
        let update_query = update(state.sid_lists.clone())
            .set(
                state.sid_description.clone(),
                SqlPart::new(SqlString::new("Updated via ORM")),
            )
            .r#where(where_eq_int(&state.sid_id, id))
            .or_where(where_eq_str(&state.sid_name, "nonexistent"));

        let sql = sql_to_string(update_query.to_sql().expect("update to_sql failed"));
        conn.execute(&sql, []).unwrap();
    })
    .await?;

    Ok(Redirect::to(&format!("/lists/{}", id)))
}

// ---------------------------------------------------------------------------
// Route: POST /lists/:id/delete-completed - DeleteQuery with or_where + limit
// ORM features: delete_from, where, or_where, limit, to_sql (on DeleteQuery)
// ---------------------------------------------------------------------------

async fn delete_completed(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Redirect, AppError> {
    let state = state.clone();
    tokio::task::spawn_blocking(move || {
        let conn = state.db.lock().unwrap();

        // delete_from with where + or_where + limit
        let del = delete_from(state.sid_todos.clone())
            .r#where({
                let b = SqlBuilder::new();
                b.append_safe("list_id = ");
                b.append_int64(id);
                b.append_safe(" AND completed = 1");
                b.accumulated()
            })
            .limit(100)
            .expect("limit failed");

        let sql = sql_to_string(del.to_sql().expect("delete to_sql failed"));
        conn.execute(&sql, []).unwrap();
    })
    .await?;

    Ok(Redirect::to(&format!("/lists/{}", id)))
}

// ---------------------------------------------------------------------------
// Route: GET /safe-ids - Demonstrate safe_identifier and SqlBuilder
// ORM features: safe_identifier, SqlBuilder, SqlFragment, append_safe,
//               append_int32, append_int64, append_float64, append_string,
//               append_fragment, accumulated, col
// ---------------------------------------------------------------------------

async fn safe_ids_demo(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let results = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let mut results: Vec<StatRow> = Vec::new();

            // safe_identifier - valid
            let valid = safe_identifier("valid_name");
            results.push(StatRow {
                label: "safe_identifier('valid_name')".to_string(),
                value: format!("ok={}", valid.is_ok()),
            });

            // safe_identifier - invalid (empty)
            let invalid = safe_identifier("");
            results.push(StatRow {
                label: "safe_identifier('')".to_string(),
                value: format!("ok={}", invalid.is_ok()),
            });

            // SqlBuilder with all append methods
            let b = SqlBuilder::new();
            b.append_safe("SELECT ");
            b.append_safe("* FROM ");
            b.append_safe("todos WHERE ");
            b.append_safe("id = ");
            b.append_int32(42);
            let frag1 = b.accumulated();
            results.push(StatRow {
                label: "SqlBuilder (append_safe + append_int32)".to_string(),
                value: sql_to_string(frag1),
            });

            // append_int64
            let b2 = SqlBuilder::new();
            b2.append_safe("id = ");
            b2.append_int64(9999999999i64);
            results.push(StatRow {
                label: "append_int64".to_string(),
                value: sql_to_string(b2.accumulated()),
            });

            // append_float64
            let b3 = SqlBuilder::new();
            b3.append_safe("price = ");
            b3.append_float64(19.99);
            results.push(StatRow {
                label: "append_float64".to_string(),
                value: sql_to_string(b3.accumulated()),
            });

            // append_string (with SQL escaping)
            let b4 = SqlBuilder::new();
            b4.append_safe("name = ");
            b4.append_string("O'Brien");
            results.push(StatRow {
                label: "append_string (with escaping)".to_string(),
                value: sql_to_string(b4.accumulated()),
            });

            // append_fragment
            let inner_b = SqlBuilder::new();
            inner_b.append_safe("completed = 1");
            let inner_frag = inner_b.accumulated();

            let b5 = SqlBuilder::new();
            b5.append_safe("SELECT * FROM todos WHERE ");
            b5.append_fragment(inner_frag);
            results.push(StatRow {
                label: "append_fragment".to_string(),
                value: sql_to_string(b5.accumulated()),
            });

            // col() for qualified column references
            let col_frag = col(state.sid_todos.clone(), state.sid_title.clone());
            results.push(StatRow {
                label: "col(todos, title)".to_string(),
                value: sql_to_string(col_frag),
            });

            results
        })
        .await?
    };

    let template = ChangesetDemoTemplate { results };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Route: GET /order-nulls - Demonstrate NullsFirst/NullsLast
// ORM features: order_by_nulls, NullsFirst, NullsLast
// ---------------------------------------------------------------------------

async fn order_nulls_demo(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let results = {
        let state = state.clone();
        tokio::task::spawn_blocking(move || {
            let mut results: Vec<StatRow> = Vec::new();

            // NullsFirst
            let q1 = from(state.sid_todos.clone())
                .order_by_nulls(
                    state.sid_due_date.clone(),
                    true,
                    NullsPosition::new(NullsFirst::new()),
                );
            results.push(StatRow {
                label: "ORDER BY due_date ASC NULLS FIRST".to_string(),
                value: sql_to_string(q1.to_sql()),
            });

            // NullsLast
            let q2 = from(state.sid_todos.clone())
                .order_by_nulls(
                    state.sid_due_date.clone(),
                    false,
                    NullsPosition::new(NullsLast::new()),
                );
            results.push(StatRow {
                label: "ORDER BY due_date DESC NULLS LAST".to_string(),
                value: sql_to_string(q2.to_sql()),
            });

            results
        })
        .await?
    };

    let template = ChangesetDemoTemplate { results };
    Ok(Html(template.render()?))
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    orm::init(None).unwrap();

    let conn = Connection::open("todo.db")?;
    init_db(&conn)?;

    let list_table = build_list_table();
    let todo_table = build_todo_table();
    let tag_table = build_tag_table();
    let todo_tag_table = build_todo_tag_table();

    let state = AppState {
        db: Arc::new(Mutex::new(conn)),
        list_table,
        todo_table,
        tag_table,
        todo_tag_table,
        sid_id: sid("id"),
        sid_name: sid("name"),
        sid_description: sid("description"),
        sid_title: sid("title"),
        sid_completed: sid("completed"),
        sid_priority: sid("priority"),
        sid_due_date: sid("due_date"),
        sid_list_id: sid("list_id"),
        sid_todo_id: sid("todo_id"),
        sid_tag_id: sid("tag_id"),
        sid_created_at: sid("created_at"),
        sid_lists: sid("lists"),
        sid_todos: sid("todos"),
        sid_tags: sid("tags"),
        sid_todo_tags: sid("todo_tags"),
    };

    let app = Router::new()
        // Core CRUD routes
        .route("/", get(index))
        .route("/lists", post(create_list))
        .route("/lists/{id}", get(show_list))
        .route("/lists/{id}/delete", post(delete_list))
        .route("/lists/{id}/todos", post(create_todo))
        .route("/todos/{id}/toggle", post(toggle_todo))
        .route("/todos/{id}/delete", post(delete_todo_handler))
        .route("/todos/{id}/edit", post(edit_todo))
        // Search
        .route("/search", get(search))
        // Stats with all aggregates
        .route("/stats", get(stats))
        // High priority with where_between, where_not_null
        .route("/lists/{id}/high-priority", get(high_priority))
        // Overdue with where_not, where_null/where_not_null
        .route("/overdue", get(overdue))
        // Combined reports: union, intersect, except
        .route("/reports/combined", get(combined_reports))
        // Subquery features: exists, subquery, where_in_subquery
        .route("/lists/{id}/has-completed", get(has_completed))
        // Bulk operations: where_in
        .route("/todos/bulk-complete", post(bulk_complete))
        // Pagination: limit, offset, count_sql
        .route("/lists/{id}/page/{page}", get(paginated_list))
        // Lock modes: ForUpdate, ForShare
        .route("/locks", get(locks_demo))
        // All join types
        .route("/joins", get(joins_demo))
        // All changeset validators
        .route("/changeset-demo", get(changeset_demo))
        // Timestamps helper
        .route("/timestamps", get(timestamps_demo))
        // Update with or_where
        .route("/lists/{id}/update-desc", post(update_list_desc))
        // Delete completed with or_where + limit
        .route("/lists/{id}/delete-completed", post(delete_completed))
        // Safe identifiers & SqlBuilder demo
        .route("/safe-ids", get(safe_ids_demo))
        // NullsFirst / NullsLast demo
        .route("/order-nulls", get(order_nulls_demo))
        // Static files
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    println!("Todo App running at http://localhost:5003");
    println!("Routes:");
    println!("  GET  /                          - Index (left_join, group_by, select_expr, count_all, col, order_by)");
    println!("  GET  /lists/:id                 - Show list (where, order_by, order_by_nulls, select)");
    println!("  POST /lists                     - Create list (changeset, cast, validate_required, validate_length)");
    println!("  POST /lists/:id/delete          - Delete list (delete_from, where, delete_sql)");
    println!("  POST /lists/:id/todos           - Create todo (changeset with ALL validators)");
    println!("  POST /todos/:id/toggle          - Toggle todo (update builder, set, where)");
    println!("  POST /todos/:id/edit            - Edit todo (changeset, put_change, get_change, delete_change)");
    println!("  POST /todos/:id/delete          - Delete todo (delete_from, where, limit)");
    println!("  GET  /search?q=...              - Search (where_like, or_where, safe_to_sql)");
    println!("  GET  /stats                     - Stats (count_all/col, sum/avg/min/max_col, group_by, having, or_having, distinct, count_sql)");
    println!("  GET  /lists/:id/high-priority   - High priority (where_between, where_not_null, limit, offset)");
    println!("  GET  /overdue                   - Overdue (where_not, where_null, where_not_null)");
    println!("  GET  /reports/combined           - Combined (union_sql, union_all_sql, intersect_sql, except_sql)");
    println!("  GET  /lists/:id/has-completed   - Subqueries (exists_sql, subquery, where_in_subquery)");
    println!("  POST /todos/bulk-complete       - Bulk complete (where_in)");
    println!("  GET  /lists/:id/page/:page      - Pagination (limit, offset, count_sql)");
    println!("  GET  /locks                     - Lock modes (ForUpdate, ForShare)");
    println!("  GET  /joins                     - All joins (inner, left, right, full, cross)");
    println!("  GET  /changeset-demo            - All changeset validators");
    println!("  GET  /timestamps                - timestamps() helper");
    println!("  GET  /safe-ids                  - safe_identifier, SqlBuilder, col");
    println!("  GET  /order-nulls               - NullsFirst / NullsLast");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5003").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
