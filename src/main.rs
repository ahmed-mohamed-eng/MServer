mod cli;

use cli::{commands::get_subcommand, flags::get_flags};

fn main() {
    let command = get_subcommand().unwrap();
    let command_flags = get_flags();

    println!(
        "command is {:?} and its flags are {:?}",
        command,
        command_flags.join(", ")
    );
}
