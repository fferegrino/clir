use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn out(filename: &str) -> Result<Box<dyn Write>> {
    let boxed: Box<dyn Write> = match filename {
        "" => Box::new(std::io::stdout()),
        _ => Box::new(File::create(filename)?),
    };
    Ok(boxed)
}
