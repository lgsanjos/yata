use db::tasks::Task;

use crate::{
    cli::{
        parser::{parse_command, Command},
        serializer::{format_tasks_for_listing, serialize_tasks_by_status},
    },
    db::crud::{connection, create, select_all, select_non_done_tasks, setup},
};

pub mod cli;
pub mod db;
pub mod parser;

fn edit_tasks(conn: &rusqlite::Connection) {
    let tasks = select_non_done_tasks(&conn);
    let mut task_serialized = format_tasks_for_listing(tasks);
    task_serialized.push_str("\nDONE:\n");

    let new_state: Result<String, std::io::Error> = edit::edit(task_serialized);
    println!("{}", new_state.unwrap());
}

fn create_task(conn: &rusqlite::Connection, args: Vec<String>) {
    let t: Task = Task {
        id: 0_i32,
        project: "".into(),
        status: "TODO".into(),
        title: args.join(" "),
    };

    create(conn, &t);
}

fn list_tasks(conn: &rusqlite::Connection, _args: Vec<String>) {
    let tasks = select_non_done_tasks(&conn);
    let output = format_tasks_for_listing(tasks);

    println!("{}", output);
}

pub fn execute_command(mut cli_args: Vec<String>) {
    let conn: rusqlite::Connection = connection();
    setup(&conn);

    cli_args.remove(0);

    match parse_command(cli_args) {
        Some(command) => match command {
            Command::New(args) => create_task(&conn, args),
            Command::List(args) => list_tasks(&conn, args),
            Command::Edit(_) => edit_tasks(&conn),
            Command::Status(_) => show_status(&conn),
        },
        _ => println!("command not found"),
    }
}

fn show_status(conn: &rusqlite::Connection) {
    let tasks = select_all(&conn);
    let output = serialize_tasks_by_status(tasks);
    println!("{}", output);
}
