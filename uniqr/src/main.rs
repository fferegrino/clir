use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about)]
struct Args {
    #[arg(value_name = "IN_FILE", default_value = "-")]
    in_file: String,
    #[arg(value_name = "OUT_FILE", default_value = "")]
    out_file: String,
    #[arg(short('c'), long("count"))]
    count: bool,
}

fn run(args: Args) -> Result<()> {
    let mut new_line = String::new();
    let mut old_line = String::new();
    let mut count = 0;
    let mut file = clir::open(&args.in_file)?;
    let mut out_file = clir::out(&args.out_file)?;
    loop {
        let read_bytes = file.read_line(&mut new_line)?;
        if read_bytes == 0 {
            break;
        }
        if old_line.trim_end() != new_line.trim_end() {
            if count > 0 {
                if args.count {
                    write!(out_file, "{count:>4} {old_line}")?;
                } else {
                    write!(out_file, "{old_line}")?;
                }
            }
            old_line = new_line.clone();
            count = 0;
        }
        count += 1;
        new_line.clear();
    }

    if old_line != "" {
        if args.count {
            write!(out_file, "{count:>4} {old_line}")?;
        } else {
            write!(out_file, "{old_line}")?;
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
