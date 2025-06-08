use anyhow::Result;
use clap::Parser;
use std::io::BufRead;

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    #[arg(value_name="FILES", default_value="-", num_args=1..)]
    files: Vec<String>,
    #[arg(value_name="LINES",short('n'), long("lines"), default_value="10", value_parser=clap::value_parser!(u64).range(1..))]
    lines: u64,
    #[arg(
        value_name="BYTES",
        short('c'),
        long("bytes"), value_parser=clap::value_parser!(u64).range(1..), conflicts_with("lines"))]
    bytes: Option<u64>,
}

fn run(args: Args) -> Result<()> {
    let file_count = args.files.iter().count();
    for filename in args.files {
        let mut file = clir::open(&filename)?;
        let mut line = String::new();
        if file_count > 1 {
            println!("==> {filename} <==")
        }
        for _ in 0..args.lines {
            let read_bytes = file.read_line(&mut line)?;
            if read_bytes == 0 {
                break;
            }
            print!("{line}");
            line.clear();
        }
    }
    Ok(())
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
