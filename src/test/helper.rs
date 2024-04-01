use crate::db::tasks::Task;

pub fn edit_tasks_input1() -> String {
    String::from("
DOING:
\tgroceries:
\t\t1\tbuy eggs
\tyat:
\t\t4\timplement commands
TODO:
\tgroceries:
\t\t0\tbuy milk
\tyat:
\t\t2\timplement statuses command
DONE:
\tgroceries:
\t\t3\tbuy flour
")
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
