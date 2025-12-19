use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use clap::Parser;

mod dumper;
mod formatter;

use crate::dumper::{HexDumper, OutputFormat};


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

    /// bytes per line
    #[arg(short, long, default_value_t = 16)]
    bytes_per_line: usize,

    /// show ascii
    #[arg(short, long, default_value_t = true)]
    show_ascii: bool,

    /// output format
    #[arg(short, long, default_value_t = OutputFormat::Canonical)]
    output_format: OutputFormat,

    /// use colors
    #[arg(short, long, default_value_t = true)]
    use_colors: bool,
}

fn text_to_binary(text: &str) -> String {
    text.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<_>>()
        .join(" ")
}



fn main() {
    let args = Args::parse();

    // Validate arguments
    if args.output.is_some() && !args.binary {
        eprintln!("--output can only be used together with --binary");
        std::process::exit(1);
    }

    // Get input file or show error
    let input_file = match args.input_file {
        Some(path) => path,
        None => {
            eprintln!("Error: Input file is required");
            eprintln!("Usage: hexaprint <INPUT_FILE> [OPTIONS]");
            std::process::exit(1);
        }
    };

    // Open the file
    let file = File::open(&input_file).unwrap_or_else(|e| {
        eprintln!("Error opening file '{}': {}", input_file, e);
        std::process::exit(1);
    });

    // Process based on mode
    if args.binary {
        // Binary mode: convert text to binary representation
        let reader = io::BufReader::new(file);
        let mut writer = args.output.as_ref().map(|path| {
            BufWriter::new(File::create(path).expect("Failed to create output file"))
        });

        for line in reader.lines() {
            let line = line.unwrap();
            let binary_line = text_to_binary(&line);
            if let Some(writer) = writer.as_mut() {
                writeln!(writer, "{}", binary_line).expect("Failed to write binary output");
            } else {
                println!("{}", binary_line);
            }
        }
    } else {
        // Hex dump mode
        let reader = io::BufReader::new(file);
        let mut dumper = HexDumper::new(
            args.bytes_per_line,
            args.show_ascii,
            args.output_format,
            args.use_colors,
        );

        dumper.dump(reader).unwrap();
    }
}
