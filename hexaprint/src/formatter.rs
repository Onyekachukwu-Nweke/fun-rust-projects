use std::io::{self, BufRead};
use crate::dumper::OutputFormat;
use crate::format::{
    format_offset,
    format_offset_uppercase,
    format_hex,
    format_hex_uppercase,
    format_ascii,
    is_printable
};

/// Dispatcher that routes to the appropriate format function based on output type
pub fn format_line_by_type(
    offset: usize,
    data: &[u8],
    format: OutputFormat,
    bytes_per_line: usize,
    show_ascii: bool,
    use_colors: bool
) {
    match format {
        OutputFormat::Canonical => format_canonical(offset, data, bytes_per_line, show_ascii, use_colors),
        OutputFormat::PlainHex => format_plain_hex(data, use_colors),
        OutputFormat::CArray => format_c_array(data),
        OutputFormat::Uppercase => format_uppercase(offset, data, bytes_per_line, show_ascii, use_colors),
    }
}

/// Format and print a line in canonical format (lowercase hex with offset)
fn format_canonical(offset: usize, data: &[u8], bytes_per_line: usize, show_ascii: bool, use_colors: bool) {
    print_line(offset, data, bytes_per_line, show_ascii, use_colors);
}

/// Format and print data as plain hex bytes (no offset, no ASCII column)
fn format_plain_hex(data: &[u8], use_colors: bool) {
    let mut output = String::new();

    for (i, byte) in data.iter().enumerate() {
        if i > 0 {
            output.push(' ');
        }
        output.push_str(&format_hex(*byte, use_colors));
    }

    println!("{}", output);
}

/// Format and print data as C-style array elements
fn format_c_array(data: &[u8]) {
    let mut output = String::new();

    for (i, byte) in data.iter().enumerate() {
        if i > 0 {
            output.push_str(", ");
        }
        output.push_str(&format!("0x{:02x}", byte));
    }

    output.push(',');  // Trailing comma for C array
    println!("{}", output);
}

/// Format and print a line in uppercase format
fn format_uppercase(offset: usize, data: &[u8], bytes_per_line: usize, show_ascii: bool, use_colors: bool) {
    let mut output = format_offset_uppercase(offset);
    output.push_str(": ");

    // Hex bytes
    for byte in data {
        output.push_str(&format_hex_uppercase(*byte, use_colors));
        output.push(' ');
    }

    // Padding
    if data.len() < bytes_per_line {
        let padding = " ".repeat((bytes_per_line - data.len()) * 3);
        output.push_str(&padding);
    }

    // ASCII column
    if show_ascii {
        output.push_str(" |");
        for byte in data {
            if is_printable(*byte) {
                output.push(*byte as char);
            } else {
                output.push('.');
            }
        }
        output.push('|');
    }

    println!("{}", output);
}

/// Read a chunk of data from the input source
pub fn read_chunk(source: &mut impl BufRead, buffer: &mut [u8]) -> io::Result<usize> {
    source.read(buffer)
}

/// Print a formatted line in canonical (lowercase) format
/// This is the main function used by the dumper
pub fn print_line(
    offset: usize,
    data: &[u8],
    bytes_per_line: usize,
    show_ascii: bool,
    use_colors: bool,
) {
    let mut output = format_offset(offset);
    output.push_str(": ");

    // Hex bytes
    for byte in data {
        output.push_str(&format_hex(*byte, use_colors));
        output.push(' ');
    }

    // Padding to align ASCII column
    if data.len() < bytes_per_line {
       let padding_spaces = (bytes_per_line - data.len()) * 3;
       output.push_str(&" ".repeat(padding_spaces));
    }

    // ASCII column
    if show_ascii {
        output.push_str(" |");
        for byte in data {
            output.push_str(&format_ascii(*byte, use_colors));
        }
        output.push_str("|");
    }

    println!("{}", output);
}
