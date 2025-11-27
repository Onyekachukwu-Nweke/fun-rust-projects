# HexaPrint - Hex Dump Utility

A command-line utility written in Rust that reads binary files and displays them in hexadecimal format, similar to `hexdump` or `xxd`.

## Project Overview

HexaPrint reads binary files byte-by-byte and displays the contents in a human-readable hexadecimal format, with ASCII representation alongside for easier analysis of binary data.

## Features

- Read and display binary files in hexadecimal format
- Show byte offsets for easy navigation
- Display ASCII representation of printable characters
- Support different output formats (canonical, plain hex, C-style array)
- Configurable bytes per line
- Color-coded output for better readability
- Support for reading from stdin

## Example Output

```
00000000: 48 65 6c 6c 6f 2c 20 57 6f 72 6c 64 21 0a 54 68  |Hello, World!.Th|
00000010: 69 73 20 69 73 20 61 20 74 65 73 74 20 66 69 6c  |is is a test fil|
00000020: 65 2e 0a                                         |e..|
```

## Pseudocode

### Main Program Flow

```
FUNCTION main():
    // Parse command-line arguments
    args = parse_arguments()

    // Determine input source (file or stdin)
    IF args.input_file IS provided:
        input_source = open_file(args.input_file)
    ELSE:
        input_source = stdin

    // Create hex dumper with configuration
    dumper = HexDumper.new(
        bytes_per_line = args.bytes_per_line OR 16,
        show_ascii = args.show_ascii OR true,
        output_format = args.format OR "canonical",
        use_colors = args.use_colors OR true
    )

    // Process and display the input
    dumper.dump(input_source)

    RETURN success
```

### HexDumper Structure

```
STRUCTURE HexDumper:
    bytes_per_line: usize
    show_ascii: bool
    output_format: OutputFormat
    use_colors: bool
    offset: usize

ENUM OutputFormat:
    Canonical    // Standard format with offset, hex, and ASCII
    PlainHex     // Only hexadecimal values
    CArray       // C-style array format
    Uppercase    // Uppercase hexadecimal
```

### Core Dumping Logic

```
FUNCTION dump(input_source):
    offset = 0
    buffer = allocate_buffer(BUFFER_SIZE)

    LOOP:
        bytes_read = read_chunk(input_source, buffer)

        IF bytes_read == 0:
            BREAK  // End of file

        // Process buffer in chunks of bytes_per_line
        FOR chunk IN buffer.chunks(bytes_per_line):
            print_line(offset, chunk)
            offset += chunk.length

    close(input_source)
```

### Line Formatting

```
FUNCTION print_line(offset, data):
    // Print offset (address)
    output = format_offset(offset)
    output += ": "

    // Print hex values
    FOR byte IN data:
        output += format_hex(byte) + " "

    // Pad if line is incomplete
    IF data.length < bytes_per_line:
        padding_spaces = (bytes_per_line - data.length) * 3
        output += " " * padding_spaces

    // Print ASCII representation
    IF show_ascii:
        output += " |"
        FOR byte IN data:
            IF is_printable(byte):
                output += char(byte)
            ELSE:
                output += "."
        output += "|"

    print(output)
```

### Helper Functions

```
FUNCTION format_offset(offset):
    // Format as 8-digit hexadecimal with leading zeros
    RETURN format!("{:08x}", offset)

FUNCTION format_hex(byte):
    // Format byte as 2-digit hexadecimal
    RETURN format!("{:02x}", byte)

FUNCTION is_printable(byte):
    // Check if byte is a printable ASCII character
    RETURN byte >= 0x20 AND byte <= 0x7E

FUNCTION read_chunk(source, buffer):
    // Read bytes into buffer
    // Return number of bytes actually read
    TRY:
        bytes_read = source.read(buffer)
        RETURN bytes_read
    CATCH error:
        handle_error(error)
        RETURN 0
```

### Command-Line Argument Parsing

```
STRUCTURE CliArgs:
    input_file: Option<String>
    bytes_per_line: Option<usize>
    format: Option<OutputFormat>
    show_ascii: bool
    use_colors: bool
    skip_bytes: Option<usize>
    length: Option<usize>

FUNCTION parse_arguments():
    // Parse using clap or similar argument parser

    DEFINE arguments:
        - input_file: positional argument (optional, uses stdin if not provided)
        - -n, --length: number of bytes to display
        - -s, --skip: skip N bytes from start
        - -c, --cols: bytes per line (default 16)
        - -f, --format: output format (canonical, plain, c-array)
        - --no-ascii: disable ASCII column
        - --no-color: disable colored output
        - -u, --uppercase: use uppercase hexadecimal

    RETURN parsed_args
```

### Color Support

```
FUNCTION colorize(byte, text):
    IF NOT use_colors:
        RETURN text

    // Color based on byte value ranges
    IF byte == 0x00:
        RETURN gray(text)          // Null bytes
    ELSE IF byte >= 0x20 AND byte <= 0x7E:
        RETURN green(text)         // Printable ASCII
    ELSE IF byte >= 0x80:
        RETURN cyan(text)          // High bytes
    ELSE:
        RETURN yellow(text)        // Control characters
```

### Advanced Features (Optional)

```
FUNCTION diff_mode(file1, file2):
    // Compare two files and highlight differences
    LOOP:
        chunk1 = read_chunk(file1)
        chunk2 = read_chunk(file2)

        FOR i IN 0..chunk1.length:
            IF chunk1[i] != chunk2[i]:
                highlight_difference(i, chunk1[i], chunk2[i])

FUNCTION search_pattern(input, pattern):
    // Search for hex pattern in file
    // Highlight matches in output

FUNCTION group_by_format():
    // Group bytes by format (e.g., 2-byte, 4-byte groups)
    // Useful for viewing structured binary data
```

## Project Structure

```
hexaprint/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           // Entry point and CLI setup
│   ├── dumper.rs         // Core hex dump logic
│   ├── formatter.rs      // Output formatting functions
│   ├── config.rs         // Configuration structures
│   └── color.rs          // Color support utilities
├── tests/
│   ├── integration_tests.rs
│   └── fixtures/         // Test binary files
└── examples/
    └── sample_usage.rs
```

## Learning Resources

### Rust Fundamentals
- [The Rust Book - Chapter 12: I/O Project](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) - Building CLI programs
- [Rust By Example - File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)
- [std::io Documentation](https://doc.rust-lang.org/std/io/) - Reading and buffering

### Binary Data & Hexadecimal
- [Understanding Hexadecimal](https://www.electronics-tutorials.ws/binary/bin_3.html)
- [Binary File Formats](https://en.wikipedia.org/wiki/Comparison_of_file_formats)
- [How to Read a Hex Dump](https://www.varonis.com/blog/how-to-use-hexdump)

### Rust Crates for This Project
- [clap](https://docs.rs/clap/) - Command-line argument parsing
- [colored](https://docs.rs/colored/) - Terminal color support
- [anyhow](https://docs.rs/anyhow/) - Error handling
- [memmap2](https://docs.rs/memmap2/) - Memory-mapped file I/O (for large files)

### Related Tools to Study
- [hexdump](https://man7.org/linux/man-pages/man1/hexdump.1.html) - Unix hex dump utility
- [xxd](https://linux.die.net/man/1/xxd) - Hex dump and reverse
- [hexyl](https://github.com/sharkdp/hexyl) - Modern hex viewer in Rust

### Specific Rust Concepts to Learn
1. **File I/O and Buffering**
   - `std::fs::File`
   - `std::io::Read` trait
   - `BufReader` for efficient reading

2. **Formatting and Display**
   - `format!` macro
   - `write!` and `writeln!` macros
   - Custom `Display` implementations

3. **Iterator Patterns**
   - `.chunks()` for processing data in blocks
   - `.enumerate()` for tracking offsets
   - `.map()` and `.filter()` for transformations

4. **Error Handling**
   - `Result<T, E>` type
   - `?` operator
   - Custom error types

5. **Command-Line Interfaces**
   - Argument parsing with `clap`
   - Reading from stdin vs files
   - Exit codes and error messages

### Advanced Topics (Optional)
- **Memory-Mapped Files**: For handling very large files efficiently
- **Async I/O**: Using `tokio` for asynchronous file reading
- **SIMD**: Vectorized operations for faster processing
- **Terminal UIs**: Using `crossterm` or `termion` for interactive features

## Implementation Milestones

### Phase 1: Basic Functionality
- [ ] Set up project structure with Cargo
- [ ] Implement basic file reading
- [ ] Display simple hex output (no formatting)
- [ ] Add offset column

### Phase 2: Standard Hex Dump Format
- [ ] Implement canonical format (offset | hex | ASCII)
- [ ] Add proper spacing and alignment
- [ ] Handle partial lines at end of file
- [ ] Add ASCII representation with non-printable character handling

### Phase 3: CLI and Configuration
- [ ] Add command-line argument parsing
- [ ] Support reading from stdin
- [ ] Implement configurable bytes per line
- [ ] Add skip and length options

### Phase 4: Enhanced Output
- [ ] Add color support
- [ ] Implement multiple output formats
- [ ] Add uppercase/lowercase hex options
- [ ] Improve error messages

### Phase 5: Advanced Features (Optional)
- [ ] File comparison mode
- [ ] Pattern searching
- [ ] Grouping by data types (2-byte, 4-byte, etc.)
- [ ] Reverse operation (hex to binary)
- [ ] Memory-mapped I/O for large files

## Testing Strategy

```
TEST basic_hex_conversion:
    input = [0x48, 0x65, 0x6C, 0x6C, 0x6F]
    output = format_hex_line(0, input)
    ASSERT output contains "48 65 6c 6c 6f"

TEST ascii_representation:
    input = [0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00]
    ascii = format_ascii(input)
    ASSERT ascii == "Hello."

TEST partial_line:
    input = [0x41, 0x42, 0x43]
    output = format_hex_line(0, input, bytes_per_line=16)
    ASSERT output is properly padded

TEST empty_file:
    ASSERT program handles empty files without errors

TEST large_file:
    // Test with file > 1GB
    ASSERT memory usage remains constant

TEST binary_file:
    // Test with actual binary files (executables, images)
    ASSERT all bytes are correctly displayed
```

## Usage Examples

```bash
# Basic usage - dump a file
hexaprint myfile.bin

# Read from stdin
cat data.bin | hexaprint

# Show only 256 bytes
hexaprint -n 256 largefile.bin

# Skip first 1024 bytes
hexaprint -s 1024 file.bin

# 8 bytes per line instead of 16
hexaprint -c 8 file.bin

# Plain hex output (no ASCII)
hexaprint --no-ascii file.bin

# C array format
hexaprint -f c-array file.bin

# Uppercase hex
hexaprint -u file.bin
```

## Expected Learning Outcomes

After completing this project, you will understand:
- How to read and process binary files in Rust
- Byte-level data manipulation and representation
- Formatting and displaying data in multiple formats
- Building robust CLI applications
- Efficient buffered I/O operations
- Error handling in file operations
- Working with different number systems (hex, decimal, binary)

## License

MIT
