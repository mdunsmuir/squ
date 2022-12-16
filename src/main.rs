use std::{
    fs::{self, File},
    io::{stdout, Write},
    path::PathBuf,
};

use clap::Parser;
use crowbook_text_processing::clean::quotes;

/// Convert plain quotes (single and double) in a file to matched "smart" quotes.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The file to convert
    file: PathBuf,

    #[arg(short, long, default_value_t = false)]
    in_place: bool,
}

fn main() {
    let args = Args::parse();
    let converted = quotes(fs::read_to_string(&args.file).unwrap());

    if args.in_place {
        File::create(&args.file)
            .unwrap()
            .write_all(converted.as_bytes())
            .unwrap();
    } else {
        stdout().write_all(converted.as_bytes()).unwrap();
    }
}

// TODO: Undo smart quotes
