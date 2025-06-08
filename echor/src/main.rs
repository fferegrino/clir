use clap::ArgAction;
use clap::Parser;

#[derive(Parser)]
#[command(version, author, about)]
struct Args {
    #[arg(value_name="TEXT", help="The text to echo", required=true, num_args(1..))]
    text: Vec<String>,
    #[arg(short='n', help="Do not print the trailing newline character", action=ArgAction::SetTrue)]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    let all_texts = args.text.join(" ");
    print!("{}", all_texts);

    if !args.omit_newline {
        println!();
    }
}
