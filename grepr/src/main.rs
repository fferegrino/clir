use anyhow::Result;
use clap::Parser;
use clir::open;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "PATTERN")]
    pattern: String,
    #[arg(value_name = "FILES", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long)]
    ignore_case: bool,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long)]
    counts: bool,
}

fn run(args: Args) -> Result<()> {
    let regex = Regex::new(&args.pattern)?;

    // println!("{regex}");
    for file in args.files {
        let mut file = open(&file)?;
        let mut line = String::new();

        loop {
            let bytes_read = file.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }

            if regex.is_match(&line) {
                println!("{}", line.trim());
            }

            line.clear();
        }
    }
    Ok(())
}

fn main() {
    if let Err(error) = run(Args::parse()) {
        eprintln!("{error}");
        return std::process::exit(1);
    }
}
