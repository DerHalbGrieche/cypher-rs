use clap::{Arg, Command};

fn new_decrypt_command() -> Command {
    let command = Command::new("decrypt")
        .arg(Arg::new("text"))
        .arg_required_else_help(true);
    command
}

fn new_encrypt_command() -> Command {
    let command = Command::new("encrypt")
        .arg(Arg::new("text"))
        .arg_required_else_help(true);
    command
}
pub fn new_command() -> Vec<Command> {
    vec![new_decrypt_command(), new_encrypt_command()]
}
