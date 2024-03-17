use rusqlite::Connection;

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub project: String,
    pub status: String,
    pub title: String,
}

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
}

pub fn create(conn: &Connection, task: &Task) -> bool {
    let res = conn.execute(
        "INSERT INTO tasks (title, status, project) VALUES (?1, ?2, ?3)",
        (&task.title, &task.status, &task.project),
    );

    res.is_ok()
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
