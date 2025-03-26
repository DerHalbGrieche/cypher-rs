use clap::ArgMatches;

pub trait Cipher {
    fn encrypt(text: &str) -> String;
    fn decrypt(text: &str) -> String;
}

pub fn handle_algorithm<T: Cipher>(sub_matches: &ArgMatches) {
    if let Some(sub_matches) = sub_matches.subcommand_matches("encrypt") {
        println!(
            "{}",
            T::encrypt(sub_matches.get_one::<String>("text").expect("required"))
        );
    } else if let Some(sub_matches) = sub_matches.subcommand_matches("decrypt") {
        println!(
            "{}",
            T::decrypt(sub_matches.get_one::<String>("text").expect("required"))
        );
    }
}
