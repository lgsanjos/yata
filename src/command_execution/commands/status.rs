use crate::{
    command_execution::persistence::crud::select_all,
    output_serializer::output_serializer::serialize_tasks_by_status,
};

pub fn show_status(conn: &rusqlite::Connection) -> String {
    let tasks = select_all(&conn);
    serialize_tasks_by_status(&tasks)
}
