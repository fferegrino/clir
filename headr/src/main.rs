use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about)]
struct Args {
    #[arg(value_name="FILES", default_value="-", num_args=1..)]
    files: Vec<String>,
    #[arg(value_name="LINES",short('n'), long("lines"), value_parser=clap::value_parser!(u64).range(1..))]
    lines: Option<u64>,
    #[arg(
        value_name="BYTES",
        short('c'),
        long("bytes"), default_value="10", value_parser=clap::value_parser!(u64).range(1..), conflicts_with("lines"))]
    bytes: u64,
}

fn main() {
    let args = Args::parse();
    println!("{args:?}");
}
