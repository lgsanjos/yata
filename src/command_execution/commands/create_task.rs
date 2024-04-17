use crate::command_execution::{models::task::Task, persistence::crud::create};

pub fn create_task(conn: &rusqlite::Connection, args: Vec<String>) -> String {
    let t: Task = Task::new(0_i32, "".into(), "TODO".into(), &args.join(" "), 0);

    let res = create(conn, &t);

    match res {
        Ok(_) => "Task created".to_string(),
        Err(err) => format!("Error: {}", err),
    }
}
