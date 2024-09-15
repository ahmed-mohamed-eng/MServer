use std::env;

pub fn get_flags() -> Vec<String> {
    let mut flags: Vec<String> = vec![];

    for arg in env::args() {
        if arg.contains("-") {
            let fmt_arg = arg.replace("-", "");
            flags.push(fmt_arg);
        } else {
            continue;
        }
    }

    flags
}
