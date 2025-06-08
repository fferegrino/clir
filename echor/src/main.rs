use clap::{Arg, ArgAction, Command};

fn main() {
    let program = Command::new("echor")
        .version("0.1.0")
        .author("Antonio Feregrino <antonio.feregrino@gmail.com>")
        .about("A simple echo program");

    let matches = program
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("The text to echo")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print the trailing newline character")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let texts = matches
        .get_many::<String>("text")
        .unwrap_or_default()
        .cloned()
        .collect::<Vec<_>>();
    let omit_newline = matches.get_flag("omit_newline");

    let all_texts = texts.join(" ");
    print!("{}", all_texts);

    if !omit_newline {
        println!();
    }
}
