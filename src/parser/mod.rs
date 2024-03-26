pub mod tokenizer;

use crate::db::tasks::Task;

use self::tokenizer::parse;

enum DiffOperation {
    RemoveTask,
    UpdateTaskFields,
    NewTask
}

pub struct TaskDiff {
    pub original_task: Option<Task>,
    pub new_task: Option<Task>,
    pub operation: DiffOperation
}

fn diff(input: &str, tasks: Vec<Task>) -> Vec<TaskDiff> {

    let parsed_tasks = parse(input);

    tasks.into_iter().map(|task| {
       TaskDiff {
            original_task: task.clone(),
            new_task: Some(task.clone()),
            operation: DiffOperation::New
        }
    }).collect()

}

