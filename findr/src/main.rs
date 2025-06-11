use clap::{Parser, ValueEnum};
use regex::Regex;

#[derive(Parser, Debug)]
#[command(about, author, version, long_about = None)]
struct Args {
    #[clap(value_name = "PATH", help = "Path to search in", default_value = ".")]
    paths: Vec<String>,

    #[arg(
        value_name = "NAME",
        short = 'n',
        long = "name",
        value_parser = Regex::new,
        action = clap::ArgAction::Append,
        num_args = 0..
    )]
    names: Vec<Regex>,

    #[arg(
        value_name = "TYPE",
        short = 't',
        long = "type",
        value_parser = clap::value_parser!(EntryType),
        action = clap::ArgAction::Append,
        num_args = 0..
    )]
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[EntryType::Dir, EntryType::File, EntryType::Link]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            EntryType::Dir => Some(clap::builder::PossibleValue::new("d")),
            EntryType::File => Some(clap::builder::PossibleValue::new("f")),
            EntryType::Link => Some(clap::builder::PossibleValue::new("l")),
        }
    }
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
