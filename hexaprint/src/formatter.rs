use std::io::{self, BufRead};
use crate::color::{colorize_byte, get_color_for_byte, colorize};

fn format_offset(offset: usize) -> String {
    format!("{:08x}", offset)
}

fn format_hex(byte: u8, use_colors: bool) -> String {
    let hex = format!("{:02x}", byte);
    colorize_byte(byte, hex, use_colors)
}

fn is_printable(byte: u8) -> bool {
    byte >= 0x20 && byte <= 0x7E
}

fn format_ascii(byte: u8, use_colors: bool) -> String {
    if is_printable(byte) {
        let ch = byte as char;
        if use_colors {
            colorize(&ch.to_string(), get_color_for_byte(byte), use_colors)
        } else {
            ch.to_string()
        }
    } else {
        ".".to_string()
    }
}

//TODO: Error handling and statement matching
pub fn read_chunk(source: &mut impl BufRead, buffer: &mut [u8]) -> io::Result<usize> {
    source.read(buffer)
}

pub fn print_line(
    offset: usize, 
    data: &[u8], 
    bytes_per_line: usize, 
    show_ascii: bool,
    use_colors: bool,
) {
    let mut output = format_offset(offset);
    output.push_str(": ");

    for byte in data {
        output.push_str(&format_hex(*byte, use_colors));
        output.push(' ');
    }

    if data.len() < bytes_per_line {
       let padding_spaces = (bytes_per_line - data.len()) * 3;
       output.push_str(&" ".repeat(padding_spaces));
    }

    if show_ascii {
        output.push_str(" |");
        for byte in data {
          if is_printable(*byte) {
            output.push_str(&format_ascii(*byte, use_colors));
          } else {
            output.push('.');
          }
        }
        output.push_str("|");
    }
    
    println!("{}", output);
}

    