use anyhow::Result;
use clap::Parser;
use std::ops::Range;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    #[arg(short, long, value_name = "BYTES", conflicts_with_all = &["chars", "fields"])]
    bytes: Option<String>,
    #[arg(short, long, conflicts_with_all = &["bytes", "fields"])]
    chars: Option<String>,
    #[arg(short, long, conflicts_with_all = &["bytes", "chars"])]
    fields: Option<String>,
    #[arg(short, long("delim"), value_name = "DELIMITER", default_value = "\t")]
    delimiter: Option<String>,
}

fn parse_number(s: &str, original_s: &str) -> Result<usize> {
    if s.starts_with('+') {
        return Err(anyhow::anyhow!("illegal list value: \"{}\"", original_s));
    }
    if s.is_empty() {
        return Err(anyhow::anyhow!("illegal list value: \"{}\"", original_s));
    }
    let value = s.parse::<usize>();
    let value = match value {
        Ok(value) => {
            if value == 0 {
                return Err(anyhow::anyhow!("illegal list value: \"{}\"", value));
            }
            Ok(value)
        }
        Err(_) => Err(anyhow::anyhow!("illegal list value: \"{}\"", original_s)),
    }?;
    Ok(value)
}

type PosRanges = Vec<Range<usize>>;

enum Extract {
    Bytes(PosRanges),
    Chars(PosRanges),
    Fields(PosRanges),
}

fn parse_pos(s: String) -> Result<PosRanges> {
    let mut ranges = Vec::new();

    if s.is_empty() {
        return Err(anyhow::anyhow!("illegal list value: \"{}\"", s));
    }

    for range in s.split(',') {
        match range.split_once('-') {
            Some((start, end)) => {
                let start = parse_number(start, &range)?;
                let end = parse_number(end, &range)?;
                if start >= end {
                    return Err(anyhow::anyhow!(
                        "First number in range ({}) must be lower than second number ({})",
                        start,
                        end
                    ));
                }
                ranges.push((start - 1)..end);
            }
            None => {
                let single_index = parse_number(range, &range)?;
                ranges.push(single_index - 1..single_index);
            }
        }
    }
    Ok(ranges)
}

fn run(args: Args) -> Result<()> {
    if args.delimiter.is_some() && args.delimiter.as_ref().unwrap().len() != 1 {
        return Err(anyhow::anyhow!(
            "--delim \"{}\" must be a single byte",
            args.delimiter.as_ref().unwrap()
        ));
    }

    let extract = if let Some(bytes) = args.bytes {
        Extract::Bytes(parse_pos(bytes)?)
    } else if let Some(chars) = args.chars {
        Extract::Chars(parse_pos(chars)?)
    } else if let Some(fields) = args.fields {
        Extract::Fields(parse_pos(fields)?)
    } else {
        return Err(anyhow::anyhow!("no extract type specified"));
    };

    for file in args.files.into_iter() {
        let mut file = clir::open(&file)?;
        match &extract {
            Extract::Chars(_ranges) => {
                let mut line = String::new();
                loop {
                    let mut line = String::new();
                    let bytes_read = file.read_line(&mut line)?;
                    if bytes_read == 0 {
                        break;
                    }
                    for range in _ranges {
                        for i in range.start..range.end {
                            print!("{}", line.chars().nth(i).unwrap());
                        }
                    }
                    println!();
                    line.clear();
                }
            }
            Extract::Bytes(_ranges) => {
                let mut line = String::new();
                loop {
                    let bytes_read = file.read_line(&mut line)?;
                    if bytes_read == 0 {
                        break;
                    }
                    let line_as_bytes = line.as_bytes();
                    let mut vector_of_bytes = Vec::new();
                    for range in _ranges {
                        for i in range.start..range.end {
                            vector_of_bytes.push(line_as_bytes[i]);
                        }
                        let string_of_bytes = String::from_utf8_lossy(&vector_of_bytes);
                        println!("{}", string_of_bytes);
                    }
                    line.clear();
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
// --------------------------------------------------
#[cfg(test)]
mod unit_tests {
    use super::parse_pos;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_pos() {
        // The empty string is an error
        assert!(parse_pos("".to_string()).is_err());

        // Zero is an error
        let res = parse_pos("0".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        let res = parse_pos("0-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "0""#);

        // A leading "+" is an error
        let res = parse_pos("+1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "+1""#,);

        let res = parse_pos("+1-2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "+1-2""#,
        );

        let res = parse_pos("1-+2".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            r#"illegal list value: "1-+2""#,
        );

        // Any non-number is an error
        let res = parse_pos("a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1,a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a""#);

        let res = parse_pos("1-a".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "1-a""#,);

        let res = parse_pos("a-1".to_string());
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), r#"illegal list value: "a-1""#,);

        // Wonky ranges
        let res = parse_pos("-".to_string());
        assert!(res.is_err());

        let res = parse_pos(",".to_string());
        assert!(res.is_err());

        let res = parse_pos("1,".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-1".to_string());
        assert!(res.is_err());

        let res = parse_pos("1-1-a".to_string());
        assert!(res.is_err());

        // First number must be less than second
        let res = parse_pos("1-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (1) must be lower than second number (1)"
        );

        let res = parse_pos("2-1".to_string());
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().to_string(),
            "First number in range (2) must be lower than second number (1)"
        );

        // All the following are acceptable
        let res = parse_pos("1".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("01".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1]);

        let res = parse_pos("1,3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("001,0003".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 2..3]);

        let res = parse_pos("1-3".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("0001-03".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..3]);

        let res = parse_pos("1,7,3-5".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![0..1, 6..7, 2..5]);

        let res = parse_pos("15,19-20".to_string());
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec![14..15, 18..20]);
    }
}
