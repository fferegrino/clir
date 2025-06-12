use regex::Regex;
use clap::Parser;
use anyhow::Result;
use walkdir::WalkDir;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "PATH", default_value = ".")]
    paths: Vec<String>,

    #[arg(
        value_name = "NAME",
        short,
        long("name"),
        value_parser = Regex::new,
        action = clap::ArgAction::Append,
        help = "Regex to match file names",
        num_args = 0..,
    )]
    names: Vec<Regex>,

    #[arg(
        short('t'),
        long("type"),
        value_name = "TYPE",
        // value_parser = |s: &str| s.to_lowercase(),
        action = clap::ArgAction::Append,
        help = "Entry type to match",
        num_args = 0..,
    )]
    entry_types: Vec<String>,
}

fn run(args: Args) -> Result<()> {
    for path in args.paths {
        for entry in WalkDir::new(path) {
            let entry = entry?;
            let name = entry.file_name().to_str().unwrap();
            let matches_name = args.names.iter().any(|re| re.is_match(name));
            // let matches_type = args.entry_types.iter().any(|t| t == entry.file_type());
            let mut matches_type = true;
            if !args.entry_types.is_empty() {
                matches_type = matches_type && (entry.file_type().is_dir() && args.entry_types.contains("d"))
                || (entry.file_type().is_file() && args.entry_types.contains("f"))
                || (entry.file_type().is_symlink() && args.entry_types.contains("l"));
                println!("{}", matches_type);
            }

            if (matches_name || args.names.is_empty()) 
            && (matches_type || args.entry_types.is_empty())
            && (
                entry.file_type().is_dir() || entry.file_type().is_file() || entry.file_type().is_symlink()) {
                println!("{}", entry.path().display());
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
