use std::env;

pub fn get_subcommand() -> Result<String, String> {
    let command = env::args().nth(1).unwrap();

    if is_flag(&command) {
        Err("Flags Can't be commands".to_string())
    } else {
        Ok(command)
    }
}

fn is_flag(command: &String) -> bool {
    command.contains("-")
}
