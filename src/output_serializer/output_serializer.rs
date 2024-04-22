use itertools::Itertools;

use crate::{
    command_execution::models::task::Task,
    input_parser::task_helper::{task_by_projects, task_by_statuses},
};

pub fn serialize_tasks(tasks: &Vec<Task>, serialiazer: fn(String, &Vec<Task>) -> String) -> String {
    let mut response: String = String::new();

    task_by_statuses(tasks)
        .into_iter()
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .for_each(|(status, tasks_by_status): (String, Vec<Task>)| {
            response.push_str(&format!("\n{}:", status));

            task_by_projects(&tasks_by_status)
                .iter()
                .sorted_by(|a, b| a.0.cmp(b.0))
                .for_each(|(project, tasks_by_project)| {
                    response.push_str(&format!(
                        "\n{}",
                        serialiazer(project.clone(), tasks_by_project)
                    ));
                });
        });

    response
}

pub fn format_tasks_for_listing(tasks: &Vec<Task>) -> String {
    serialize_tasks(tasks, |project, tasks| {
        let tasks_output = tasks.iter().fold(String::new(), |acc, task| {
            format!("{}\n    {}  {}", acc, task.id, task.title)
        });

        format!("  {}:{}", project, tasks_output)
    })
}

pub fn serialize_tasks_by_status(tasks: &Vec<Task>) -> String {
    serialize_tasks(tasks, |project, tasks| {
        format!("  {}: {}", project, tasks.len())
    })
}

#[cfg(test)]
#[test]
fn test_serialize_tasks_by_status() {
    use crate::test::helper::create_task_list1;
    let tasks = create_task_list1();

    let output = serialize_tasks_by_status(&tasks);
    let expected = "
DOING:
  groceries: 2
  yat: 1
DONE:
  groceries: 1
TODO:
  groceries: 1
  yat: 1";

    assert_eq!(expected, output);
}

