use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use clap::Parser;

#[derive(Parser)]
#[command(name = "hexaprint", about = "Hex dump utility", long_about = None)]
struct Args {
    /// Input file to read from
    input_file: Option<String>,

    /// convert to binary
    #[arg(short, long)]
    binary: bool,

    /// Optional file to write binary output into
    #[arg(short, long)]
    output: Option<String>,
}

fn text_to_binary(text: &str) -> String {
    text.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let args = Args::parse();
    let input_file = args.input_file.unwrap_or("".to_string());
    let file = File::open(input_file).unwrap();
    let reader = io::BufReader::new(file);
    if args.output.is_some() && !args.binary {
        eprintln!("--output can only be used together with --binary");
        std::process::exit(1);
    }
    let mut writer = args.output.as_ref().map(|path| {
        BufWriter::new(File::create(path).expect("Failed to create output file"))
    });
    for line in reader.lines() {
        let line = line.unwrap();
        if args.binary {
            let binary_line = text_to_binary(&line);
            if let Some(writer) = writer.as_mut() {
                writeln!(writer, "{}", binary_line).expect("Failed to write binary output");
            } else {
                println!("{}", binary_line);
            }
        } else {
            println!("{}", line);
        }
    }
}
