mod atbash;
mod base64;
mod caesar;
mod command_factory;
mod config;
mod matching;
mod rot13;
mod rsa;
mod substitution;
mod tripple_des;

use clap::command;
use command_factory::new_command;
use matching::handle_algorithm;

fn main() {
    let matches = command!()
        .subcommands([
            command!("3DES").subcommands(new_command()),
            command!("base64").subcommands(new_command()),
            command!("atbash").subcommands(new_command()),
            command!("caesar").subcommands(new_command()),
            command!("rot13").subcommands(new_command()),
            command!("rsa").subcommands(new_command()),
            command!("substitution").subcommands(new_command()),
        ])
        .subcommand_required(true)
        .arg_required_else_help(true)
        .get_matches();

    match matches.subcommand() {
        Some(("3DES", sub_matches)) => handle_algorithm::<tripple_des::TrippleDes>(sub_matches),
        Some(("base64", sub_matches)) => handle_algorithm::<base64::Base64>(sub_matches),
        Some(("atbash", sub_matches)) => handle_algorithm::<atbash::Atbash>(sub_matches),
        Some(("caesar", sub_matches)) => handle_algorithm::<caesar::Caesar>(sub_matches),
        Some(("rot13", sub_matches)) => handle_algorithm::<rot13::Rot13>(sub_matches),
        Some(("rsa", sub_matches)) => handle_algorithm::<rsa::RsaCommand>(sub_matches),
        Some(("substitution", sub_matches)) => {
            handle_algorithm::<substitution::Substitution>(sub_matches)
        }
        _ => {}
    }
}
