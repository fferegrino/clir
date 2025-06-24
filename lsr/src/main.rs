use anyhow::Result;
use clap::Parser;
use std::{fs, fs::read_dir, path::PathBuf};

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "FILES", default_value = ".")]
    paths: Vec<String>,
    #[arg(short = 'a', long = "all")]
    show_hidden: bool,
    #[arg(short = 'l', long = "long")]
    long_format: bool,
}

fn find_files(paths: &[String], show_hidden: bool) -> Result<Vec<PathBuf>> {
    let mut found_paths: Vec<PathBuf> = Vec::new();

    for path in paths {
        let path_metadata = std::fs::metadata(path);
        match path_metadata {
            Err(e) => eprintln!("{path}: {e}"),
            Ok(metadata) => {
                if metadata.is_dir() {
                    let entries = fs::read_dir(path)?;
                    for entry in entries {
                        let entry = entry?;
                        let path = entry.path();
                        let is_hidden_file = path
                            .file_name()
                            .map_or(false, |fname| fname.to_string_lossy().starts_with('.'));

                        if is_hidden_file && !show_hidden {
                            continue;
                        }
                        found_paths.push(entry.path());
                    }
                } else {
                    found_paths.push(PathBuf::from(&path));
                }
            }
        }
    }

    Ok(found_paths)
}

fn run(args: Args) -> Result<()> {
    let files = find_files(&args.paths, args.show_hidden)?;
    for file in files {
        println!("{}", file.file_name().unwrap().to_string_lossy());
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
