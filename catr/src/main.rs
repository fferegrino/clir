use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "FILES", default_value = "-", help = "Input file(s)")]
    files: Vec<String>,
    #[arg(short('n'), long("number"), help = "Number lines")]
    number: bool,
    #[arg(short('b'), long("number-nonblank"), help = "Number non-blank lines")]
    number_nonblank: bool,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        let file = open(&filename);
        if let Err(err) = file {
            eprintln!("Failed to open {filename}: {err}");
            continue;
        }

        let mut file = file.unwrap();
        let mut line = String::new();
        let mut line_count = 1;
        loop {
            let bytes = file.read_line(&mut line).unwrap();
            if bytes == 0 {
                break;
            }
            let clean_line = line.trim_end();
            if args.number {
                println!("{line_count:>6}\t{clean_line}");
                line_count = line_count + 1;
            } else if args.number_nonblank {
                if clean_line != "" {
                    println!("{line_count:>6}\t{clean_line}");
                    line_count = line_count + 1;
                } else {
                    println!("")
                }
            } else {
                println!("{clean_line}");
            }
            line.clear();
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
