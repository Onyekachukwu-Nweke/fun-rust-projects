#[derive(Debug)]
struct HexDumper {
  bytes_per_line: usize,
  show_ascii: bool,
  output_format: OutputFormat,
  use_colors: bool,
  offset: usize,
}

enum OutputFormat {
  Canonical,
  PlainHex,
  CArray,
  Uppercase,
}

impl HexDumper {
  fn new(bytes_per_line: usize, show_ascii: bool, output_format: OutputFormat, use_colors: bool) -> HexDumper {
    HexDumper {
      bytes_per_line,
      show_ascii,
      output_format,
      use_colors,
      offset: 0,
    }
  }

  fn dump(&mut self, input: impl BufRead) -> io::Result<()> {
    let mut offset = 0;
    let mut buffer = [0; 16];
    loop {
      let bytes_read = input.read(&mut buffer)?;
      if bytes_read == 0 {
        break;
      }
      self.offset += bytes_read;
      self.dump_line(&buffer[..bytes_read]);
    }
    Ok(())
  }
}
    