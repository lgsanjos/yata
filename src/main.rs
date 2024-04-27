use std::env;

use crate::{
    command_execution::{
        commands::{
            create_task::create_task, edit_tasks::edit_tasks, list_tasks::list_tasks,
            status::show_status,
        },
        persistence::crud::{connection, setup},
    },
    config::load_config_file,
    input_parser::input_parser::{parse_command, Command},
};

pub mod command_execution;
pub mod config;
pub mod input_parser;
pub mod output_serializer;
pub mod test;

pub fn display_error_if_needed(res: Result<usize, rusqlite::Error>) {
    match res {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

fn execute_command(mut cli_args: Vec<String>) -> String {
    let parse_config = load_config_file("~/.yat/config.toml".to_string());
    match parse_config {
        Ok(_) => (),
        Err(err) => return format!("Error: {}", err),
    }
    let config = parse_config.unwrap();

    let conn: rusqlite::Connection = connection(&config);
    display_error_if_needed(setup(&conn));

    cli_args.remove(0);

    match parse_command(cli_args) {
        Some(command) => match command {
            Command::New(args) => create_task(&conn, args),
            Command::List(args) => list_tasks(&conn, args),
            Command::Edit(_) => edit_tasks(&conn),
            Command::Status(_) => show_status(&conn),
            Command::Version => "0.0.1".to_string(),
        },
        _ => "command not found".to_string(),
    }
}

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let output = execute_command(cli_args);

    println!("{}", output);
}
