use std::collections::HashMap;

use crate::db::tasks::Task;

pub fn task_by_projects(tasks: Vec<Task>) -> HashMap<String, Vec<Task>> {
    let mut hash: HashMap<String, Vec<Task>> = HashMap::new();

    tasks.iter().fold(&mut hash, |acc, task| {
        let status = task.project.clone();
        let tasks = acc.entry(status).or_insert(vec![]);
        tasks.push(task.clone());
        acc
    });

    return hash;
}

pub fn task_by_statuses(tasks: Vec<Task>) -> HashMap<String, Vec<Task>> {
    let mut hash: HashMap<String, Vec<Task>> = HashMap::new();

    tasks.iter().fold(&mut hash, |acc, task| {
        let status = task.status.clone();
        let tasks = acc.entry(status).or_insert(vec![]);
        tasks.push(task.clone());
        acc
    });

    return hash;
}

#[test]
fn test_task_by_projects() {
    let tasks = vec![
        Task::new(0, "groceries", "TODO", "buy milk"),
        Task::new(1, "groceries", "DOING", "buy eggs"),
        Task::new(2, "yat", "TODO", "implement statuses command"),
        Task::new(3, "groceries", "DONE", "buy eggs"),
        Task::new(4, "yat", "DOING", "implement commands"),
    ];

    let output = task_by_projects(tasks);

    assert_eq!(
        output.get("groceries").unwrap(),
        &vec![
            Task::new(0, "groceries", "TODO", "buy milk"),
            Task::new(1, "groceries", "DOING", "buy eggs"),
            Task::new(3, "groceries", "DONE", "buy eggs"),
        ]
    );
    assert_eq!(
        output.get("yat").unwrap(),
        &vec![
            Task::new(2, "yat", "TODO", "implement statuses command"),
            Task::new(4, "yat", "DOING", "implement commands"),
        ]
    );
}

#[test]
fn test_task_by_statuses() {
    let tasks = vec![
        Task::new(0, "groceries", "TODO", "buy milk"),
        Task::new(1, "groceries", "DOING", "buy eggs"),
        Task::new(2, "yat", "TODO", "implement statuses command"),
        Task::new(3, "groceries", "DONE", "buy eggs"),
        Task::new(4, "yat", "DOING", "implement commands"),
    ];

    let output = task_by_statuses(tasks);

    assert_eq!(
        output.get("DOING").unwrap(),
        &vec![
            Task::new(1, "groceries", "DOING", "buy eggs"),
            Task::new(4, "yat", "DOING", "implement commands"),
        ]
    );
    assert_eq!(
        output.get("DONE").unwrap(),
        &vec![Task::new(3, "groceries", "DONE", "buy eggs")]
    );
    assert_eq!(
        output.get("TODO").unwrap(),
        &vec![
            Task::new(0, "groceries", "TODO", "buy milk"),
            Task::new(2, "yat", "TODO", "implement statuses command"),
        ]
    );
}

