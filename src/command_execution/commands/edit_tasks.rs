use crate::{
    command_execution::{
        persistence::crud::{create, delete, select_non_done_tasks, update},
        task_diff::{
            diff::{diff, DiffOperation, TaskDiff},
            serializer::serialize,
        },
    },
    output_serializer::output_serializer::format_tasks_for_listing,
};

pub fn edit_tasks(conn: &rusqlite::Connection) -> String {
    let tasks = select_non_done_tasks(&conn);
    let mut task_serialized = format_tasks_for_listing(&tasks);
    task_serialized.push_str("\nDONE:\n");

    let input: String = edit::edit(task_serialized).unwrap();
    let tasks_diff = diff(&input, &tasks);

    tasks_diff
        .iter()
        .for_each(|diff: &TaskDiff| process(conn, diff));

    serialize(&tasks_diff)
}

fn process(conn: &rusqlite::Connection, diff: &TaskDiff) {
    let res = match diff.operation {
        DiffOperation::RemoveTask => delete(conn, &diff.original_task.clone().unwrap()),
        DiffOperation::UpdateTaskFields => update(
            conn,
            &diff.original_task.clone().unwrap(),
            &diff.new_task.clone().unwrap(),
        ),
        DiffOperation::NewTask => create(conn, &diff.original_task.clone().unwrap()),
        DiffOperation::DoNothing => Ok(0),
    };

    match res {
        Ok(_) => {}
        Err(err) => eprintln!("Error: {}", err),
    }
}
