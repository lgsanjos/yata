use crate::{
    command_execution::persistence::crud::{select_done_tasks, select_non_done_tasks},
    output_serializer::output_serializer::format_tasks_for_listing,
};

pub fn list_tasks(conn: &rusqlite::Connection, args: Vec<String>) -> String {
    let is_done = args.iter().find(|&a| a == "--done");

    let tasks = match is_done {
        Some(_) => select_done_tasks(conn),
        None => select_non_done_tasks(conn),
    };

    format_tasks_for_listing(&tasks)
}
