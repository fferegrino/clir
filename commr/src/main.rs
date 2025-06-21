use anyhow::{Result, bail};
use clap::Parser;
use std::io::BufRead;

#[derive(Parser, Debug)]
struct Args {
    #[arg(value_name = "FILE1", default_value = "-")]
    file1: String,
    #[arg(value_name = "FILE2", default_value = "-")]
    file2: String,
    #[arg(short('1'))]
    hide_col1: bool,
    #[arg(short('2'))]
    hide_col2: bool,
    #[arg(short('3'))]
    hide_col3: bool,
    #[arg(short('i'))]
    insensitive: bool,
    #[arg(long("output-delimiter"), short('d'), default_value = "\t")]
    delimiter: String,
}

fn print(args: &Args, col1: Option<String>, col2: Option<String>, col3: Option<String>) {
    let mut v = Vec::new();
    if let Some(value) = col1 {
        if !args.hide_col1 {
            v.push(value);
        }
    } else if let Some(value) = col2 {
        if !args.hide_col2 {
            if !args.hide_col1 {
                let empty = String::from("");
                v.push(empty);
            }
            v.push(value)
        }
    } else if let Some(value) = col3 {
        if !args.hide_col3 {
            if !args.hide_col1 {
                let empty = String::from("");
                v.push(empty);
            }
            if !args.hide_col2 {
                let empty = String::from("");
                v.push(empty);
            }
            v.push(value)
        }
    }
    if !v.is_empty() {
        println!("{}", v.join(&args.delimiter));
    }
}

fn run(args: Args) -> Result<()> {
    // println!("{args:?}");
    if args.file1 == "-" && args.file2 == "-" {
        bail!(r#"Both input files cannot be STDIN ("-")"#)
    }
    let mut lines1 = clir::open(&args.file1)?
        .lines()
        .map_while(Result::ok)
        .map(|st| {
            if args.insensitive {
                return st.to_lowercase().trim().to_string();
            }
            return st.trim().to_string();
        });
    let mut lines2 = clir::open(&args.file2)?
        .lines()
        .map_while(Result::ok)
        .map(|st| {
            if args.insensitive {
                return st.to_lowercase().trim().to_string();
            }
            return st.trim().to_string();
        });

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();
    loop {
        match (&line1, &line2) {
            (Some(l1), Some(l2)) => {
                if l1 == l2 {
                    print(&args, None, None, Some(l1.to_string()));
                    line1 = lines1.next();
                    line2 = lines2.next();
                } else if l1 < l2 {
                    print(&args, Some(l1.to_string()), None, None);
                    line1 = lines1.next();
                } else {
                    print(&args, None, Some(l2.to_string()), None);
                    line2 = lines2.next();
                }
            }
            (None, Some(l2)) => {
                print(&args, None, Some(l2.to_string()), None);
                line2 = lines2.next();
            }
            (Some(l1), None) => {
                print(&args, Some(l1.to_string()), None, None);
                line1 = lines1.next();
            }
            (None, None) => {
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(error) = run(Args::parse()) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
