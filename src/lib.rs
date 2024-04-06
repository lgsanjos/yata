use db::{
    crud::{delete, update},
    tasks::Task,
};
use task_diff::diff::DiffOperation::{DoNothing, NewTask, RemoveTask, UpdateTaskFields};

use crate::{
    cli::serializer::{format_tasks_for_listing, serialize_tasks_by_status},
    db::crud::{create, select_all, select_non_done_tasks},
    task_diff::{
        diff::{diff, TaskDiff},
        serializer::serialize,
    },
};

pub mod cli;
pub mod db;
pub mod task_diff;
pub mod test;

pub fn edit_tasks(conn: &rusqlite::Connection) -> String {
    let tasks = select_non_done_tasks(&conn);
    let mut task_serialized = format_tasks_for_listing(&tasks);
    task_serialized.push_str("\nDONE:\n");

    let input: String = edit::edit(task_serialized).unwrap();
    let tasks_diff = diff(&input, &tasks);

    tasks_diff
        .iter()
        .for_each(|diff: &TaskDiff| match diff.operation {
            RemoveTask => display_result(delete(conn, &diff.original_task.clone().unwrap())),
            UpdateTaskFields => display_result(update(
                conn,
                &diff.original_task.clone().unwrap(),
                &diff.new_task.clone().unwrap(),
            )),
            NewTask => display_result(create(conn, &diff.original_task.clone().unwrap())),
            DoNothing => (),
        });

    serialize(&tasks_diff)
}

pub fn display_result(res: Result<usize, rusqlite::Error>) {
    match res {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

pub fn create_task(conn: &rusqlite::Connection, args: Vec<String>) -> String {
    let t: Task = Task {
        id: 0_i32,
        project: "".into(),
        status: "TODO".into(),
        title: args.join(" "),
    };

    let res = create(conn, &t);
    display_result(res);

    "Task created".to_string()
}

pub fn list_tasks(conn: &rusqlite::Connection, _args: Vec<String>) -> String {
    let tasks = select_non_done_tasks(&conn);
    format_tasks_for_listing(&tasks)
}

pub fn show_status(conn: &rusqlite::Connection) -> String {
    let tasks = select_all(&conn);
    serialize_tasks_by_status(&tasks)
}
