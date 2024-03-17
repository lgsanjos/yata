use crate::db::crud::{connection, create, select_all, setup, Task};
use std::env;

mod db;

fn create_task(conn: &rusqlite::Connection) {
    let title: Result<String, std::io::Error> = edit::edit("hello add your tasks here");

    let t: Task = Task {
        id: 0_i32,
        project: "".into(),
        status: "TODO".into(),
        title: title.unwrap(),
    };

    println!("{:?}", t);

    create(conn, &t);
}

fn list_tasks(conn: &rusqlite::Connection) {
    println!("{:?}", select_all(conn));
}

fn main() {
    let conn: rusqlite::Connection = connection();
    setup(&conn);

    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let command: String = args.get(1).unwrap().to_owned();

    match command.clone().as_ref() {
        "add" | "a" => create_task(&conn),
        "list" | "l" => list_tasks(&conn),
        _ => println!("command not found"),
    }
}
