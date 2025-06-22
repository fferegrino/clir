use anyhow::{Result, anyhow};
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

#[derive(Parser)]
struct Args {
    #[arg(value_name = "FILES", default_value = "-")]
    files: Vec<String>,
    #[arg(short('n'), long, default_value = "10")]
    lines: String,
    #[arg(short('c'), long, conflicts_with = "lines")]
    bytes: Option<String>,
    #[arg(short, long)]
    quiet: bool,
}

#[derive(PartialEq, Eq, Debug)]
enum Action {
    Everything,
    From(i64),
}

fn parse_quantity(qty: String) -> Result<Action> {
    Ok(if qty.starts_with("+") {
        let parsed = qty[1..].parse::<i64>()?;
        if parsed == 0 {
            Action::Everything
        } else {
            Action::From(parsed)
        }
    } else if qty.starts_with("-") {
        let parsed = qty[1..].parse::<i64>()?;
        if parsed == 0 {
            Action::Everything
        } else {
            Action::From(-1 * parsed)
        }
    } else {
        Action::From(-1 * qty.parse::<i64>()?)
    })
}

fn counts(file_name: &str) -> Result<(i64, i64)> {
    let mut line = String::new();
    let mut line_count: i64 = 0;
    let mut byte_count: i64 = 0;
    let mut file = clir::open(&file_name)?;
    loop {
        let bc = file.read_line(&mut line)? as i64;
        if bc == 0 {
            break;
        }
        line_count += 1;
        byte_count += bc;

        line.clear();
    }

    return Ok((line_count, byte_count));
}

fn run(args: Args) -> Result<()> {
    let multiple_files = args.files.iter().count();

    let lines = parse_quantity(args.lines).map_err(|e| anyhow!("illegal byte count -- {e}"))?;
    let bytes = args
        .bytes
        .map(parse_quantity)
        .transpose()
        .map_err(|e| anyhow!("illegal line count -- {e}"))?;

    for (idx, file_name) in args.files.iter().enumerate() {
        let mut file = clir::open(&file_name)?;
        let (line_count, _byte_count) = counts(&file_name)?;
        if multiple_files > 1 && !args.quiet {
            if idx > 0 {
                println!()
            }
            println!("==> {} <==", file_name);
        }
        if let Some(bt) = &bytes {
            match bt {
                Action::Everything => {
                    let mut l = String::new();
                    loop {
                        let b = file.read_line(&mut l)?;
                        if b == 0 {
                            break;
                        }
                        print!("{}", l);
                        l.clear();
                    }
                }
                Action::From(count) => {
                    if *count != 0 {
                        let skip_until = if *count < 0 {
                            let negative = _byte_count + count;
                            if negative < 0 { 0 } else { negative }
                        } else {
                            if *count > _byte_count { 0 } else { count - 1 }
                        };
                        let mut ff = BufReader::new(File::open(&file_name)?);
                        ff.seek(SeekFrom::Start(skip_until as u64))?;
                        let mut bbytes = Vec::new();
                        ff.read_to_end(&mut bbytes)?;

                        let srtr = String::from_utf8_lossy(&bbytes);
                        if !bbytes.is_empty() {
                            print!("{}", srtr);
                        }
                    }
                }
            }
        } else {
            match lines {
                Action::Everything => {
                    let mut l = String::new();
                    loop {
                        let b = file.read_line(&mut l)?;
                        if b == 0 {
                            break;
                        }
                        print!("{}", l);
                        l.clear();
                    }
                }
                Action::From(count) => {
                    if count != 0 && line_count >= count {
                        let skip_until = if count < 0 {
                            let negative = line_count + count;
                            if negative < 0 { 0 } else { negative }
                        } else {
                            if count > line_count { 0 } else { count - 1 }
                        };
                        let mut idx = 0;
                        let mut l = String::new();
                        loop {
                            let b = file.read_line(&mut l)?;
                            if b == 0 {
                                break;
                            }
                            if idx >= skip_until {
                                print!("{}", l);
                            }
                            idx += 1;
                            l.clear();
                        }
                    }
                }
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

#[cfg(test)]
mod tests {
    use super::Action;
    use super::parse_quantity;
    use anyhow::Result;
    use pretty_assertions::assert_eq;

    fn test_positive() -> Result<()> {
        let parsed = parse_quantity("10".to_string())?;
        assert_eq!(parsed, Action::From(-10));
        Ok(())
    }

    fn test_positive_from() -> Result<()> {
        let parsed = parse_quantity("+10".to_string())?;
        assert_eq!(parsed, Action::From(10));
        Ok(())
    }

    fn test_plus_zero() -> Result<()> {
        let parsed = parse_quantity("+0".to_string())?;
        assert_eq!(parsed, Action::Everything);
        Ok(())
    }
}
