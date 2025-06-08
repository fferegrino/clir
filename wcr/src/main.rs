use anyhow::Result;
use clap::ArgAction;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version, author)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, action=ArgAction::SetTrue)]
    lines: bool,
    #[arg(short, long, action=ArgAction::SetTrue)]
    words: bool,
    #[arg(short('c'), long, action=ArgAction::SetTrue)]
    bytes: bool,
    #[arg(short('m'), long, conflicts_with("bytes"))]
    chars: bool,
}

fn pv(args: &Args, lines: usize, words: usize, bytes: usize, chars: usize, filename: &str) {
    if args.lines {
        print!("{lines:>8}")
    }
    if args.words {
        print!("{words:>8}")
    }
    if args.bytes {
        print!("{bytes:>8}")
    }
    if args.chars {
        print!("{chars:>8}")
    }
    if filename != "-" {
        print!(" {filename}")
    }

    println!()
}

fn run(mut args: Args) -> Result<()> {
    if [args.words, args.lines, args.bytes, args.chars]
        .iter()
        .all(|flag| !flag)
    {
        args.lines = true;
        args.words = true;
        args.bytes = true;
    }

    let mut total_bytes_read = 0;
    let mut total_lines_read = 0;
    let mut total_words_read = 0;
    let mut total_chars_read = 0;
    let file_count = args.files.len();

    for filename in &args.files {
        let mut file = clir::open(&filename)?;
        let mut bytes_read_total: usize = 0;
        let mut lines_read_total: usize = 0;
        let mut chars_read_total: usize = 0;
        let mut words_read_total: usize = 0;
        let mut line = String::new();
        loop {
            let bytes_read = file.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            let words = line.split_whitespace();
            words_read_total += words.count();
            bytes_read_total += bytes_read;
            lines_read_total += 1;
            chars_read_total += line.chars().count();
            line.clear();
        }

        pv(
            &args,
            lines_read_total,
            words_read_total,
            bytes_read_total,
            chars_read_total,
            &filename,
        );

        total_bytes_read += bytes_read_total;
        total_lines_read += lines_read_total;
        total_words_read += words_read_total;
        total_chars_read += chars_read_total;
    }
    if file_count > 1 {
        pv(
            &args,
            total_lines_read,
            total_words_read,
            total_bytes_read,
            total_chars_read,
            &"total",
        );
    }
    Ok(())
}

fn main() {
    if let Err(err) = run(Args::parse()) {
        eprintln!("{err}");
        std::process::exit(1);
    }
}
