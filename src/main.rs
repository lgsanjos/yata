use std::env;

use yat::{create_task, display_result, edit_tasks, list_tasks, show_status};

use crate::{
    command_execution::persistence::crud::{connection, setup},
    input_parser::input_parser::{parse_command, Command},
};

pub mod command_execution;
pub mod input_parser;
pub mod output_serializer;
pub mod test;

fn execute_command(mut cli_args: Vec<String>) -> String {
    let conn: rusqlite::Connection = connection();
    display_result(setup(&conn));

    cli_args.remove(0);

    match parse_command(cli_args) {
        Some(command) => match command {
            Command::New(args) => create_task(&conn, args),
            Command::List(args) => list_tasks(&conn, args),
            Command::Edit(_) => edit_tasks(&conn),
            Command::Status(_) => show_status(&conn),
        },
        _ => "command not found".to_string(),
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let output = execute_command(cli_args);

    println!("{}", output);
}
