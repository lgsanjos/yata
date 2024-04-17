use crate::command_execution::models::task::Task;

pub fn edit_tasks_input1() -> String {
    String::from(
        "
DOING:
  groceries:
    1  buy eggs
    8  buy sugar
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
        Task::new(0, "groceries", "TODO", "buy milk", 0),
        Task::new(1, "groceries", "DOING", "buy eggs", 0),
        Task::new(8, "groceries", "DOING", "buy sugar", 1),
        Task::new(2, "yat", "TODO", "implement statuses command", 0),
        Task::new(3, "groceries", "DONE", "buy flour", 0),
        Task::new(4, "yat", "DOING", "implement commands", 0),
    ]
}
