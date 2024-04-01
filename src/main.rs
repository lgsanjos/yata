use yat::execute_command;
use std::env;

pub mod db;
pub mod cli;
pub mod parser;
pub mod test;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    let output = execute_command(cli_args);

    println!("{}", output);
}
