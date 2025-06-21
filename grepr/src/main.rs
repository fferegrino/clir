use anyhow::Result;
use clap::Parser;
use clir::open;
use regex::{Regex, RegexBuilder};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "PATTERN")]
    pattern: String,
    #[arg(value_name = "FILES", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long("insensitive"))]
    ignore_case: bool,
    #[arg(short, long)]
    recursive: bool,
    #[arg(short, long("count"))]
    counts: bool,
}

fn print_file_stats(
    needle: &Regex,
    file_name: &str,
    file_count: usize,
    show_counts: bool,
) -> Result<()> {
    let mut file = open(&file_name)?;
    let mut line = String::new();
    let mut matching_lines = Vec::new();
    loop {
        let bytes_read = file.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        if needle.is_match(&line) {
            let trans = line.clone();
            matching_lines.push(trans.to_string());
        }
        line.clear();
    }

    // if matching_lines.is_empty() {
    //     return Ok(())
    // }

    if show_counts {
        if file_count > 1 {
            println!("{}:{}", file_name, matching_lines.iter().count());
        } else {
            println!("{}", matching_lines.iter().count());
        }
    } else {
        for ml in matching_lines {
            if file_count > 1 {
                print!("{}:{}", file_name, ml);
            } else {
                print!("{}", ml);
            }
        }
    }
    Ok(())
}

fn process_dir(needle: &Regex, dir: &str, file_count: usize, show_counts: bool) -> Result<()> {
    for entry in WalkDir::new(dir) {
        let entry = entry?;
        let name = entry.path().display().to_string();
        // file_count += 1;
        if !entry.file_type().is_dir() {
            // println!("Stats for {}", name);
            print_file_stats(needle, &name, 10, show_counts)?;
        }
    }
    Ok(())
}

fn run(args: Args) -> Result<()> {
    let needle = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()?;

    let file_count = args.files.iter().count();
    // let mut files_to_process = Vec::from_iter(args.files);

    for file_name in args.files.iter() {
        if file_name == "-" {
            print_file_stats(&needle, &file_name, file_count, args.counts)?;
        } else {
            let meta = std::fs::metadata(file_name)?;
            if meta.is_dir() {
                eprintln!("{file_name} is a directory");
                process_dir(&needle, &file_name, file_count, args.counts)?;
            } else {
                print_file_stats(&needle, &file_name, file_count, args.counts)?;
            }
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
