use yat::execute_command;
use std::env;

mod cli;

fn main() {
    let cli_args: Vec<String> = env::args().collect();
    execute_command(cli_args);
}
