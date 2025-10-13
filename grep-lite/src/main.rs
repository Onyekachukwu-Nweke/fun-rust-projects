use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::prelude::*;
use regex::Regex;
use clap::Parser;

/// Mini Grep Implementation
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// search pattern to match
    pattern: String,

    /// file to search
    input: Option<String>,
}

fn process_line<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        match re.find(&line) {
            Some(_) => println!("{}", line),
            None => ()
        }
    }
}

fn main() {
    let args = Args::parse();

    let re = Regex::new(args.pattern.as_str()).unwrap();
    let input = args.input;

    if let Some(filename) = input {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);
        process_line(reader, re);
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_line(reader, re);
    }
}