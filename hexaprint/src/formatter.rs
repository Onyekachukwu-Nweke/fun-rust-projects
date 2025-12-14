use std::io::{self, BufRead};

fn format_offset(offset: usize) -> String {
    format!("{:08x}", offset)
}

fn format_hex(byte: u8) -> String {
    format!("{:02x}", byte)
}

fn is_printable(byte: u8) -> bool {
    byte >= 0x20 && byte <= 0x7E
}

fn format_ascii(byte: u8) -> String {
    if is_printable(byte) {
        format!("{:c}", byte)
    } else {
        format!(".")
    }
}

//TODO: Error handling and statement matching
fn read_chunk(source: impl BufRead, buffer: &mut [u8]) -> io::Result<usize> {
    source.read(buffer)
}

fn print_line(offset: usize, data: &[u8], bytes_per_line: usize, show_ascii: bool) {
    let mut output = format_offset(offset);
    output.push_str(": ");

    for byte in data {
        output.push_str(&format_hex(*byte));
        output.push(' ');
    }

    if data.len() < bytes_per_line {
       padding_spaces = (bytes_per_line - data.len()) * 3;
       output.push_str(" " * padding_spaces);
    }

    if show_ascii {
        output.push_str(" |");
        for byte in data {
          if is_printable(*byte) {
            output.push_str(&format_ascii(*byte));
          } else {
            output.push('.');
          }
        }
        output.push_str("|");
    }
    
    println!("{}", output);
}

    