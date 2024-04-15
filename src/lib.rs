use crate::{
    command_execution::{
        models::task::Task,
        persistence::crud::{
            create, delete, select_all, select_done_tasks, select_non_done_tasks, update,
        },
        task_diff::{
            diff::{diff, DiffOperation, TaskDiff},
            serializer::serialize,
        },
    },
    output_serializer::output_serializer::{format_tasks_for_listing, serialize_tasks_by_status},
};

pub mod command_execution;
pub mod input_parser;
pub mod output_serializer;
pub mod test;

pub fn display_result(res: Result<usize, rusqlite::Error>) {
    match res {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

pub fn edit_tasks(conn: &rusqlite::Connection) -> String {
    let tasks = select_non_done_tasks(&conn);
    let mut task_serialized = format_tasks_for_listing(&tasks);
    task_serialized.push_str("\nDONE:\n");

    let input: String = edit::edit(task_serialized).unwrap();
    let tasks_diff = diff(&input, &tasks);

    tasks_diff
        .iter()
        .for_each(|diff: &TaskDiff| match diff.clone().operation {
            DiffOperation::RemoveTask => {
                display_result(delete(conn, &diff.original_task.clone().unwrap()))
            }
            DiffOperation::UpdateTaskFields => display_result(update(
                conn,
                &diff.original_task.clone().unwrap(),
                &diff.new_task.clone().unwrap(),
            )),
            DiffOperation::NewTask => {
                display_result(create(conn, &diff.original_task.clone().unwrap()))
            }
            DiffOperation::DoNothing => (),
        });

    serialize(&tasks_diff)
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

pub fn list_tasks(conn: &rusqlite::Connection, args: Vec<String>) -> String {
    let is_done = args.iter().find(|&a| a == "--done");

    let tasks = match is_done {
        Some(_) => select_done_tasks(conn),
        None => select_non_done_tasks(conn),
    };

    format_tasks_for_listing(&tasks)
}

pub fn show_status(conn: &rusqlite::Connection) -> String {
    let tasks = select_all(&conn);
    serialize_tasks_by_status(&tasks)
}
