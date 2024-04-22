use rusqlite::Connection;

use crate::{command_execution::models::task::Task, config::Config};

pub fn connection(config: &Config) -> Connection {
    Connection::open(&config.db_path).unwrap()
}

pub fn setup(conn: &Connection) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id      INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            status  TEXT NOT NULL,
            project TEXT NOT NULL,
            line_number INTEGER NOT NULL DEFAULT 0,
            weight INTEGER NOT NULL DEFAULT 0,
            duration INTEGER NOT NULL DEFAULT 0,
            created_at datetime default current_timestamp NOT NULL,
            updated_at datetime default current_timestamp NOT NULL
        )",
        (), // empty list of parameters.
    )

    // let tasks = vec![
    //     Task::new(1, "groceries", "TODO", "buy milk"),
    //     Task::new(2, "groceries", "DOING", "buy eggs"),
    //     Task::new(3, "yat", "TODO", "implement statuses command"),
    //     Task::new(4, "groceries", "DONE", "buy eggs"),
    //     Task::new(5, "yat", "DOING", "implement commands"),
    // ];
    //
    // tasks.iter().for_each(|task| {
    //     create(&conn, &task);
    // });
}

pub fn create(conn: &Connection, task: &Task) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "INSERT INTO tasks (title, status, project, line_number) VALUES (?1, ?2, ?3, ?4)",
        (&task.title, &task.status, &task.project, &task.line_number),
    )
}

pub fn delete(conn: &Connection, task: &Task) -> Result<usize, rusqlite::Error> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", [task.id])
}

pub fn update(
    conn: &Connection,
    before_task: &Task,
    after_task: &Task,
) -> Result<usize, rusqlite::Error> {
    conn.execute(
        "UPDATE tasks SET title = ?1, project = ?2, status = ?3, line_number = ?4, updated_at = current_timestamp WHERE id = ?5",
        (
            &after_task.title,
            &after_task.project,
            &after_task.status,
            &after_task.line_number,
            before_task.id,
        ),
    )
}

pub fn select_all(conn: &Connection) -> Vec<Task> {
    select(
        conn,
        "SELECT id, title, status, project, line_number FROM tasks",
    )
}

fn select(conn: &Connection, query: &str) -> Vec<Task> {
    let mut stmt: rusqlite::Statement<'_> = conn.prepare(query).unwrap();

    let tasks_iter = stmt
        .query_map([], |row| {
            let id: i32 = row.get(0)?;
            let title: String = row.get(1)?;
            let status: String = row.get(2)?;
            let project: String = row.get(3)?;
            let line_number: u32 = row.get(4)?;

            Ok(Task::new(id, &project, &status, &title, line_number))
        })
        .unwrap();

    let mut tasks: Vec<Task> = vec![];

    tasks_iter.for_each(|x: Result<Task, rusqlite::Error>| {
        let task = x.unwrap();
        tasks.push(task);
    });

    tasks
}

pub fn select_non_done_tasks(conn: &Connection) -> Vec<Task> {
    select(
        conn,
        "SELECT id, title, status, project, line_number FROM tasks where status != 'DONE' ORDER BY line_number ASC",
    )
}

pub fn select_done_tasks(conn: &Connection) -> Vec<Task> {
    select(conn, "SELECT id, title, status, project, line_number FROM tasks where status = 'DONE' and updated_at > datetime('now', '-7 day') ORDER BY line_number ASC")
}
