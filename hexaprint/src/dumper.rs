use std::io::{BufRead, Result};
use std::fmt;
use std::cmp::min;
use crate::formats::{read_chunk, format_line_by_type};
use clap::ValueEnum;

#[derive(Debug)]
pub struct HexDumper {
  bytes_per_line: usize,
  show_ascii: bool,
  output_format: OutputFormat,
  use_colors: bool,
  offset: usize,
  skip_bytes: Option<usize>,
  length_bytes: Option<usize>,
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
    length_bytes: Option<usize>,
    skip_bytes: Option<usize>,
  ) -> HexDumper {
    HexDumper {
      bytes_per_line,
      show_ascii,
      output_format,
      use_colors,
      length_bytes,
      skip_bytes,
      offset: 0,
    }
  }

  pub fn dump(
    &mut self,
    mut input: impl BufRead,
    // skip_bytes: Option<usize>,
    // length_bytes: Option<usize>,
  ) -> Result<()> {
    // Manually skip bytes by reading and discarding (works for both files and stdin)
    if let Some(skip) = self.skip_bytes {
      let mut remaining = skip;
      let mut discard_buffer = vec![0; min(BUFFER_SIZE, skip)];

      while remaining > 0 {
        let to_read = min(discard_buffer.len(), remaining);
        let bytes_read = input.read(&mut discard_buffer[..to_read])?;
        if bytes_read == 0 {
          break; // Reached EOF before skipping all bytes
        }
        remaining -= bytes_read;
      }
    }
    
    let mut bytes_remaning = self.length_bytes.unwrap_or(usize::MAX);
    let mut offset = self.offset;
    let mut buffer = vec![0; BUFFER_SIZE];

    loop {
      let read_size = min(buffer.len(), bytes_remaning);
      if read_size == 0 {
        break;
      }

      let bytes_read = read_chunk(&mut input, &mut buffer[0..read_size])?;
      if bytes_read == 0 {
        break;
      }

      for chunk in buffer[..bytes_read].chunks(self.bytes_per_line) {
        format_line_by_type(
          offset,
          chunk,
          self.output_format,
          self.bytes_per_line,
          self.show_ascii,
          self.use_colors,
        );
        offset += chunk.len();
      }

      bytes_remaning -= bytes_read;
    }

    Ok(())
  }
}
    