use crate::command_execution::models::task::Task;

pub fn edit_tasks_input1() -> String {
    String::from(
        "
DOING:
  groceries:
    1  buy eggs
  yat:
    4  implement commands
TODO:
  groceries:
    0  buy milk
  yat:
    2  implement statuses command
DONE:
  groceries:
    3  buy flour
",
    )
}

pub fn create_task_list1() -> Vec<Task> {
    vec![
        Task::new(0, "groceries", "TODO", "buy milk"),
        Task::new(1, "groceries", "DOING", "buy eggs"),
        Task::new(2, "yat", "TODO", "implement statuses command"),
        Task::new(3, "groceries", "DONE", "buy flour"),
        Task::new(4, "yat", "DOING", "implement commands"),
    ]
}
