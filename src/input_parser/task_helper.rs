use std::collections::HashMap;

use crate::command_execution::models::task::Task;

pub fn task_by_projects(tasks: &[Task]) -> HashMap<String, Vec<Task>> {
    let mut hash: HashMap<String, Vec<Task>> = HashMap::new();

    tasks.iter().fold(&mut hash, |acc, task| {
        let status = task.project.clone();
        let tasks = acc.entry(status).or_insert(vec![]);
        tasks.push(task.clone());
        acc
    });

    hash
}

pub fn task_by_statuses(tasks: &Vec<Task>) -> HashMap<String, Vec<Task>> {
    let mut hash: HashMap<String, Vec<Task>> = HashMap::new();

    tasks.iter().fold(&mut hash, |acc, task| {
        let status = task.status.clone();
        let tasks = acc.entry(status).or_insert(vec![]);
        tasks.push(task.clone());
        acc
    });

    hash
}

#[cfg(test)]
#[test]
fn test_task_by_projects() {
    let tasks = vec![
        Task::new(0, "groceries", "TODO", "buy milk", 0),
        Task::new(1, "groceries", "DOING", "buy eggs", 0),
        Task::new(2, "yat", "TODO", "implement statuses command", 0),
        Task::new(3, "groceries", "DONE", "buy eggs", 0),
        Task::new(4, "yat", "DOING", "implement commands", 0),
    ];

    let output = task_by_projects(&tasks);

    assert_eq!(
        output.get("groceries").unwrap(),
        &vec![
            Task::new(0, "groceries", "TODO", "buy milk", 0),
            Task::new(1, "groceries", "DOING", "buy eggs", 0),
            Task::new(3, "groceries", "DONE", "buy eggs", 0),
        ]
    );
    assert_eq!(
        output.get("yat").unwrap(),
        &vec![
            Task::new(2, "yat", "TODO", "implement statuses command", 0),
            Task::new(4, "yat", "DOING", "implement commands", 0),
        ]
    );
}

#[test]
fn test_task_by_statuses() {
    let tasks = crate::test::helper::create_task_list1();
    let output = task_by_statuses(&tasks);

    assert_eq!(
        output.get("DOING").unwrap(),
        &vec![
            Task::new(1, "groceries", "DOING", "buy eggs", 0),
            Task::new(8, "groceries", "DOING", "buy sugar", 1),
            Task::new(4, "yat", "DOING", "implement commands", 0),
        ]
    );
    assert_eq!(
        output.get("DONE").unwrap(),
        &vec![Task::new(3, "groceries", "DONE", "buy flour", 0)]
    );
    assert_eq!(
        output.get("TODO").unwrap(),
        &vec![
            Task::new(0, "groceries", "TODO", "buy milk", 0),
            Task::new(2, "yat", "TODO", "implement statuses command", 0),
        ]
    );
}
