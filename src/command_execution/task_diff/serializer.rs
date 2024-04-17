use itertools::Itertools;

use super::diff::{DiffOperation, TaskDiff};

pub fn serialize(diff: &Vec<TaskDiff>) -> String {
    let mut result = String::new();

    diff.iter()
        .into_group_map_by(|task_diff| task_diff.original_task.clone().unwrap().status)
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .for_each(|(status, task_diffs)| {
            result.push_str(&format!("{}:\n", status));

            task_diffs
                .iter()
                .into_group_map_by(|task_diff| task_diff.original_task.clone().unwrap().project)
                .into_iter()
                .sorted_by(|a, b| a.0.cmp(&b.0))
                .for_each(|(project, task_diffs2)| {
                    result.push_str(&format!("  {}:\n", project));

                    task_diffs2.iter().for_each(|task_diff| {
                        let serialized_diff = match task_diff.operation {
                            DiffOperation::RemoveTask => serialize_remove_diff(&task_diff),
                            DiffOperation::UpdateTaskFields => {
                                serialize_update_task_fields_diff(&task_diff)
                            }
                            DiffOperation::NewTask => serialize_new_task_diff(&task_diff),
                            DiffOperation::DoNothing => serialize_do_nothing_diff(&task_diff),
                        };

                        result.push_str(&serialized_diff);
                    });
                });
        });

    return result;
}

fn serialize_remove_diff(task_diff: &TaskDiff) -> String {
    let original_task = task_diff.original_task.as_ref().unwrap();
    format!("    - {}  {}\n", original_task.id, original_task.title)
}

fn serialize_update_task_fields_diff(task_diff: &TaskDiff) -> String {
    let original_task = task_diff.original_task.as_ref().unwrap();
    let new_task = task_diff.new_task.as_ref().unwrap();

    format!("    ~ {}  {}\n", original_task.id, new_task.title)
}

fn serialize_new_task_diff(task_diff: &TaskDiff) -> String {
    let original_task = task_diff.original_task.as_ref().unwrap();

    format!("    + {}\n", original_task.title)
}

fn serialize_do_nothing_diff(task_diff: &TaskDiff) -> String {
    let original_task = task_diff.original_task.as_ref().unwrap();
    format!("      {}  {}\n", original_task.id, original_task.title)
}

#[cfg(test)]
#[test]
fn test_serialize() {
    use crate::command_execution::models::task::Task;

    let task_diffs = vec![
        TaskDiff {
            original_task: Some(Task::new(1, "yat", "DOING", "title1", 0)),
            new_task: Some(Task::new(1, "yat", "DOING", "title1 - updated", 0)),
            operation: DiffOperation::UpdateTaskFields,
        },
        TaskDiff {
            original_task: Some(Task::new(2, "yat", "TODO", "title2", 0)),
            new_task: Some(Task::new(2, "project2", "status2", "title2", 0)),
            operation: DiffOperation::DoNothing,
        },
        TaskDiff {
            original_task: Some(Task::new(3, "groceries", "DOING", "title3", 0)),
            new_task: None,
            operation: DiffOperation::RemoveTask,
        },
        TaskDiff {
            original_task: Some(Task::new(4, "groceries", "TODO", "title4", 0)),
            new_task: None,
            operation: DiffOperation::NewTask,
        },
    ];

    let serialized_diff = serialize(&task_diffs);
    let expected = "DOING:
  groceries:
    - 3  title3
  yat:
    ~ 1  title1 - updated
TODO:
  groceries:
    + title4
  yat:
      2  title2
";
    assert_eq!(serialized_diff, expected);
}
