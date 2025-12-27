use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use clap::Parser;

mod dumper;
mod format;
mod formatter;
mod color;

use crate::dumper::{HexDumper, OutputFormat};


#[derive(Parser)]
#[command(name = "hexaprint", about = "Hex dump utility", long_about = None)]
struct Args {
    /// Input file to read from (reads from stdin if not provided)
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

    /// Number of bytes to display
    #[arg(short = 'n', long)]
    length_bytes: Option<usize>,

    /// Skip N bytes from start
    #[arg(short = 's', long)]
    skip_bytes: Option<usize>,
}

fn text_to_binary(text: &str) -> String {
    text.chars()
        .map(|c| format!("{:08b}", c as u8))
        .collect::<Vec<_>>()
        .join(" ")
}

fn get_input_source(input_file_option: Option<String>) -> Box<dyn BufRead> {
    match input_file_option {
        Some(path) => {
            // Open file with error handling
            let file = File::open(&path).unwrap_or_else(|e| {
                eprintln!("Error opening file '{}': {}", path, e);
                std::process::exit(1);
            });
            Box::new(io::BufReader::new(file))
        }
        None => {
            // Read from stdin when no file is provided
            let stdin = io::stdin();
            Box::new(io::BufReader::new(stdin.lock()))
        }
    }
}


fn main() {
    let args = Args::parse();

    // Validate arguments
    if args.output.is_some() && !args.binary {
        eprintln!("--output can only be used together with --binary");
        std::process::exit(1);
    }

    // Get input source (file or stdin)
    let reader = get_input_source(args.input_file);

    // Process based on mode
    if args.binary {
        // Binary mode: convert text to binary representation
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
        let mut dumper = HexDumper::new(
            args.bytes_per_line,
            args.show_ascii,
            args.output_format,
            args.use_colors,
            args.length_bytes,
            args.skip_bytes,
        );

        dumper.dump(reader).unwrap();
    }
}
