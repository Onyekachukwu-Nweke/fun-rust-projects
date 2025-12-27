use crate::color::{get_color_for_byte, colorize};

/// Format an offset as an 8-digit lowercase hexadecimal string
pub fn format_offset(offset: usize) -> String {
    format!("{:08x}", offset)
}

/// Format an offset as an 8-digit uppercase hexadecimal string
pub fn format_offset_uppercase(offset: usize) -> String {
    format!("{:08X}", offset)
}

/// Check if a byte is a printable ASCII character
pub fn is_printable(byte: u8) -> bool {
    byte >= 0x20 && byte <= 0x7E
}

/// Format a byte as a 2-digit hexadecimal string with optional color
pub fn format_hex(byte: u8, use_colors: bool) -> String {
    let hex = format!("{:02x}", byte);
    if use_colors {
        let color = get_color_for_byte(byte);
        colorize(&hex, color, use_colors)
    } else {
        hex
    }
}

/// Format a byte as a 2-digit uppercase hexadecimal string with optional color
pub fn format_hex_uppercase(byte: u8, use_colors: bool) -> String {
    let hex = format!("{:02X}", byte);
    if use_colors {
        let color = get_color_for_byte(byte);
        colorize(&hex, color, use_colors)
    } else {
        hex
    }
}

/// Format a byte as an ASCII character (printable) or '.' (non-printable) with optional color
pub fn format_ascii(byte: u8, use_colors: bool) -> String {
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