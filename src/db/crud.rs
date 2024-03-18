use rusqlite::Connection;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Task {
    pub id: i32,
    pub project: String,
    pub status: String,
    pub title: String,
}

impl Task {
    pub fn new(id: i32, project: &str, status: &str, title: &str) -> Self {
        Self {
            id,
            project: project.to_string(),
            status: status.to_string(),
            title: title.to_string(),
        }
    }
}

pub fn connection() -> Connection {
    Connection::open("./.yat.db").unwrap()
}

pub fn setup(conn: &Connection) -> bool {
    conn.execute(
        "DROP TAVEL tasks;
         CREATE TABLE IF NOT EXISTS tasks (
            id      INTEGER PRIMARY KEY,
            title   TEXT NOT NULL,
            status  TEXT NOT NULL,
            project TEXT NOT NULL
        )",
        (), // empty list of parameters.
    );

    let tasks = vec![
        Task::new(0, "groceries", "TODO", "buy milk"),
        Task::new(1, "groceries", "DOING", "buy eggs"),
        Task::new(2, "yat", "TODO", "implement statuses command"),
        Task::new(3, "groceries", "DONE", "buy eggs"),
        Task::new(4, "yat", "DOING", "implement commands"),
    ];

    tasks.iter().for_each(|task| {
        create(&conn, &task);
    });

    return true;
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
