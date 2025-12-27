pub enum Color {
  BrightBlack,
  Green,
  Cyan,
  Yellow
}

pub fn get_color_for_byte(byte: u8) -> Color {
  match byte {
    0x00 => Color::BrightBlack,
    0x20..=0x7E => Color::Green,
    0x80..=0xFF => Color::Cyan,
    _ => Color::Yellow,
  }
}

pub fn colorize(text: &str, color: Color, use_colors: bool) -> String {
  if !use_colors {
    return text.to_string()
  }

  let ansi_code = match color {
    Color::BrightBlack => "\x1b[90m",
    Color::Green => "\x1b[32m",
    Color::Cyan => "\x1b[36m",
    Color::Yellow => "\x1b[33m",
  };
  
  let reset_code = "\x1b[0m";

  return format!("{}{}{}", ansi_code, text, reset_code)
}
