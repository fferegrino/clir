use anyhow::Result;
use clap::{Parser, ValueEnum};
use regex::Regex;
use walkdir::WalkDir;

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

fn run(args: Args) -> Result<()> {
    for path in args.paths {
        for entry in WalkDir::new(path) {
            match entry {
                Ok(entry) => {
                    let match_type = args.entry_types.is_empty()
                        || ((entry.file_type().is_dir()
                            && args.entry_types.contains(&EntryType::Dir))
                            || (entry.file_type().is_file()
                                && args.entry_types.contains(&EntryType::File))
                            || (entry.file_type().is_symlink()
                                && args.entry_types.contains(&EntryType::Link)));

                    let name_match = args.names.is_empty()
                        || args.names.iter().any(|name_regex| {
                            name_regex.is_match(entry.file_name().to_str().unwrap())
                        });

                    if match_type && name_match {
                        println!("{}", entry.path().display())
                    }
                }
                Err(error) => {
                    eprintln!("{error}");
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
