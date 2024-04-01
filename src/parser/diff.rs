use crate::db::tasks::Task;

use super::tokenizer::{parse, ParserTask};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum DiffOperation {
    RemoveTask,
    UpdateTaskFields,
    NewTask,
    DoNothing,
}

#[derive(Debug, Clone)]
pub struct TaskDiff {
    pub original_task: Option<Task>,
    pub new_task: Option<Task>,
    pub operation: DiffOperation,
}

pub fn diff(input: &str, tasks: &Vec<Task>) -> Vec<TaskDiff> {
    let parsed_tasks = parse(input);

    // New Tasks
    let mut new_tasks: Vec<TaskDiff> = parsed_tasks
        .iter()
        .filter_map(|parser_task: &ParserTask|
            if parser_task.id.is_none() {
                Some(TaskDiff {
                    new_task: None,
                    original_task: Some(Task::new(
                        0,
                        &parser_task.project,
                        &parser_task.status,
                        &parser_task.title,
                    )),
                    operation: DiffOperation::NewTask,
                })
            } else {
                None
            }
        )
        .collect();

    // Update and Delete Tasks
    let mut delete_or_update_tasks: Vec<TaskDiff> = tasks
        .iter()
        .filter_map(|task| {
            let matching_new_task = parsed_tasks
                .iter()
                .find(|parser_task| parser_task.id.unwrap() == task.id);

            match matching_new_task {
                Some(parsed_task) => {
                    if parsed_task.title == task.title {
                        return Some(TaskDiff {
                            original_task: Some(task.clone()),
                            new_task: None,
                            operation: DiffOperation::DoNothing,
                        });
                    }

                    let new_task = Task::new(
                        task.id,
                        &parsed_task.project,
                        &parsed_task.status,
                        &parsed_task.title,
                    );

                    Some(TaskDiff {
                        original_task: Some(task.clone()),
                        new_task: Some(new_task.clone()),
                        operation: DiffOperation::UpdateTaskFields,
                    })
                }
                None => Some(TaskDiff {
                    original_task: Some(task.clone()),
                    new_task: None,
                    operation: DiffOperation::RemoveTask,
                }),
            }
        })
        .collect();

    let mut res: Vec<TaskDiff> = vec![];
    res.append(&mut new_tasks);
    res.append(&mut delete_or_update_tasks);
    return res;
}

#[cfg(test)]
#[test]
fn test_diff_empty_values() {
    let diffs = diff("", &vec![]);
    assert_eq!(diffs.len(), 0);
}

#[test]
fn test_diff_new_task() {
    let diffs = diff("TODO:\n\tacme:\n\t\tnew task 123\n", &vec![]);
    assert_eq!(diffs.len(), 1);
    assert_eq!(&diffs[0].operation, &DiffOperation::NewTask);

    let original_task = diffs[0].original_task.as_ref().unwrap();
    assert_eq!(&original_task.title, "new task 123");
    assert_eq!(&original_task.status, "TODO");
    assert_eq!(&original_task.project, "acme");
    assert_eq!(&original_task.id, &0);

    assert!(diffs[0].new_task.is_none());
}

#[test]
fn test_diff_edit_task() {
    let task = Task::new(1, "acme", "TODO", "new task 123");
    let diffs = diff("TODO:\n\tacme:\n\t\t1\tediting task 123\n", &vec![task]);

    assert_eq!(diffs.len(), 1);
    assert_eq!(&diffs[0].operation, &DiffOperation::UpdateTaskFields);
    let original_task = diffs[0].original_task.as_ref().unwrap();
    assert_eq!(&original_task.title, "new task 123");
    assert_eq!(&original_task.status, "TODO");
    assert_eq!(&original_task.project, "acme");
    assert_eq!(&original_task.id, &1);

    let new_task = diffs[0].new_task.as_ref().unwrap();
    assert_eq!(&new_task.title, "editing task 123");
    assert_eq!(&new_task.status, "TODO");
    assert_eq!(&new_task.project, "acme");
    assert_eq!(&new_task.id, &1);
}

#[test]
fn test_diff_with_no_changes() {
    use crate::test::helper::{create_task_list1, edit_tasks_input1};

    let diffs = diff(&edit_tasks_input1(), &create_task_list1());
    assert_eq!(diffs.len(), 5);

    diffs.iter().for_each(|diff| {
        assert_eq!(diff.operation, DiffOperation::DoNothing);
    });
}
