use rusqlite::Connection;

use super::tasks::Task;

pub fn connection() -> Connection {
    Connection::open("./.yat.db").unwrap()
}

pub fn setup(conn: &Connection) -> bool {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id      INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            status  TEXT NOT NULL,
            project TEXT NOT NULL
        )",
        (), // empty list of parameters.
    )
    .is_ok()

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
        "INSERT INTO tasks (title, status, project) VALUES (?1, ?2, ?3)",
        (&task.title, &task.status, &task.project),
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
        "UPDATE tasks SET title = ?1, project = ?2, status = ?3 WHERE id = ?4",
        (
            &after_task.title,
            &after_task.project,
            &after_task.status,
            before_task.id,
        ),
    )
}

pub fn select_all(conn: &Connection) -> Vec<Task> {
    let mut stmt: rusqlite::Statement<'_> = conn
        .prepare("SELECT id, title, status, project FROM tasks")
        .unwrap();

    let tasks_iter = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                project: row.get(3)?,
            })
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
    let mut stmt: rusqlite::Statement<'_> = conn
        .prepare("SELECT id, title, status, project FROM tasks where status != 'DONE'")
        .unwrap();

    let tasks_iter = stmt
        .query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                project: row.get(3)?,
            })
        })
        .unwrap();

    let mut tasks: Vec<Task> = vec![];

    tasks_iter.for_each(|x: Result<Task, rusqlite::Error>| {
        let task = x.unwrap();
        tasks.push(task);
    });

    tasks
}
