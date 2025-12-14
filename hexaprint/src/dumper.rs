use std::io::{BufRead, Result};
use std::fmt;
use crate::formatter::{read_chunk, print_line};
use clap::ValueEnum;

#[derive(Debug)]
pub struct HexDumper {
  bytes_per_line: usize,
  show_ascii: bool,
  output_format: OutputFormat,
  use_colors: bool,
  offset: usize,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
  Canonical,
  PlainHex,
  CArray,
  Uppercase,
}

impl fmt::Display for OutputFormat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      OutputFormat::Canonical => write!(f, "canonical"),
      OutputFormat::PlainHex => write!(f, "plain-hex"),
      OutputFormat::CArray => write!(f, "c-array"),
      OutputFormat::Uppercase => write!(f, "uppercase"),
    }
  }
}

const BUFFER_SIZE: usize = 8192;

impl HexDumper {
  pub fn new(
    bytes_per_line: usize,
    show_ascii: bool,
    output_format: OutputFormat,
    use_colors: bool,
  ) -> HexDumper {
    HexDumper {
      bytes_per_line,
      show_ascii,
      output_format,
      use_colors,
      offset: 0,
    }
  }

  pub fn dump(&mut self, mut input: impl BufRead) -> Result<()> {
    let mut offset = self.offset;
    let mut buffer = vec![0; BUFFER_SIZE];

    loop {
      let bytes_read = read_chunk(&mut input, &mut buffer)?;

      if bytes_read == 0 {
        break;
      }

      for chunk in buffer[..bytes_read].chunks(self.bytes_per_line) {
        print_line(
          offset,
          chunk,
          self.bytes_per_line,
          self.show_ascii,
        );
        offset += chunk.len();
      }
    }

    Ok(())
  }
}
    