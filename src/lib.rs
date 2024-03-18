use crate::{
    cli::parser::{parse_command, Command},
    db::crud::{connection, create, select_all, select_non_done_tasks, setup, Task},
};
use std::collections::HashMap;

mod cli;
mod db;

fn edit_tasks(conn: &rusqlite::Connection) {
    let tasks = select_non_done_tasks(&conn);
    let mut task_serialized = format_tasks_for_listing(tasks);
    task_serialized.push_str("\nDONE:\n");

    let new_state: Result<String, std::io::Error> = edit::edit(task_serialized);
    println!("{}", new_state.unwrap());
}

fn create_task(conn: &rusqlite::Connection, args: Vec<String>) {
    let t: Task = Task {
        id: 0_i32,
        project: "".into(),
        status: "TODO".into(),
        title: args.join(" "),
    };

    create(conn, &t);
}

fn list_tasks(conn: &rusqlite::Connection, args: Vec<String>) {
    let tasks = select_non_done_tasks(&conn);
    let output = format_tasks_for_listing(tasks);

    println!("{}", output);
}

fn format_tasks_for_listing(tasks: Vec<Task>) -> String {
    serialize_tasks(tasks, |project, tasks| {
        let tasks_output = tasks.iter().fold(String::new(), |acc, task| {
            format!("{}\n\t\t{}\t{:?}", acc, task.id, task.title)
        });

        format!("\t{}:{}", project, tasks_output)
    })
}

pub fn execute_command(mut cli_args: Vec<String>) {
    let conn: rusqlite::Connection = connection();
    setup(&conn);

    cli_args.remove(0);

    match parse_command(cli_args) {
        Some(command) => match command {
            Command::New(args) => create_task(&conn, args),
            Command::List(args) => list_tasks(&conn, args),
            Command::Edit(_) => edit_tasks(&conn),
            Command::Status(_) => show_status(&conn),
        },
        _ => println!("command not found"),
    }
}

fn serialize_tasks_for_status(tasks: Vec<Task>) -> String {
    serialize_tasks(tasks, |project, tasks| {
        format!("\t{}: {}", project, tasks.len())
    })
}

fn show_status(conn: &rusqlite::Connection) {
    let tasks = select_all(&conn);
    let output = serialize_tasks_for_status(tasks);
    println!("{}", output);
}

fn serialize_tasks(tasks: Vec<Task>, serialiazer: fn(String, &Vec<Task>) -> String) -> String {
    // Todo:
    //     groceries:
    //     yat:
    // Doing:
    //     yat:
    // Done:
    //     yat:

    let mut response: String = String::new();

    task_by_statuses(tasks)
        .into_iter()
        .for_each(|(status, tasks_by_status)| {
            response.push_str(&format!("\n{}:", status));

            task_by_projects(tasks_by_status)
                .iter()
                .for_each(|(project, tasks_by_project)| {
                    response.push_str(&format!(
                        "\n{}",
                        serialiazer(project.clone(), &tasks_by_project)
                    ));
                    // format!("\t{}: {}\n", project, tasks_by_project.len()));
                });
        });

    return response;
}

fn task_by_projects(tasks: Vec<Task>) -> HashMap<String, Vec<Task>> {
    let mut hash: HashMap<String, Vec<Task>> = HashMap::new();

    tasks.iter().fold(&mut hash, |acc, task| {
        let status = task.project.clone();
        let tasks = acc.entry(status).or_insert(vec![]);
        tasks.push(task.clone());
        acc
    });

    return hash;
}

fn task_by_statuses(tasks: Vec<Task>) -> HashMap<String, Vec<Task>> {
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

#[test]
fn test_show_status() {
    let tasks = vec![
        Task::new(0, "groceries", "TODO", "buy milk"),
        Task::new(1, "groceries", "DOING", "buy eggs"),
        Task::new(2, "yat", "TODO", "implement statuses command"),
        Task::new(3, "groceries", "DONE", "buy eggs"),
        Task::new(4, "yat", "DOING", "implement commands"),
    ];

    let output = serialize_tasks_for_status(tasks);

    let expected = "Status [\"hello\", \"world\"]\n\n";

    assert_eq!(expected, output);
}
