use std::env;
use yat::execute_command;

pub mod cli;
pub mod db;
pub mod parser;
pub mod test;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let output = execute_command(cli_args);

    println!("{}", output);
}
