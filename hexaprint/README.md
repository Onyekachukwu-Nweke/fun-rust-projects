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

### Phase 3 Implementation: Reading from stdin

```
FUNCTION get_input_source(input_file_option):
    // Determine input source: file or stdin
    MATCH input_file_option:
        Some(path):
            // Open file with error handling
            TRY:
                file = File::open(path)?
                RETURN Ok(BufReader::new(file))
            CATCH error:
                eprintln!("Error opening file '{}': {}", path, error)
                exit(1)

        None:
            // Read from stdin when no file is provided
            stdin = io::stdin()
            RETURN Ok(BufReader::new(stdin.lock()))

// Updated main flow with stdin support
FUNCTION main():
    args = parse_arguments()

    // Get input from file or stdin
    input_source = get_input_source(args.input_file)

    // Create and run dumper
    dumper = HexDumper::new(...)
    dumper.dump(input_source)?
```

### Phase 3 Implementation: Skip and Length Options

```
FUNCTION apply_skip_bytes(input_source, skip_bytes):
    // Skip N bytes from the beginning
    IF skip_bytes > 0:
        TRY:
            // Read and discard skip_bytes
            discard_buffer = vec![0; skip_bytes]
            input_source.read_exact(&mut discard_buffer)?
        CATCH error:
            eprintln!("Error skipping {} bytes: {}", skip_bytes, error)
            RETURN Err(error)

    RETURN Ok(())

FUNCTION dump_with_length_limit(input_source, length_limit):
    offset = 0
    buffer = allocate_buffer(BUFFER_SIZE)
    bytes_remaining = length_limit

    LOOP:
        // Calculate how many bytes to read this iteration
        bytes_to_read = min(buffer.length, bytes_remaining)

        IF bytes_to_read == 0:
            BREAK  // Reached length limit

        // Read chunk (only up to bytes_to_read)
        bytes_read = read_chunk(input_source, &mut buffer[0..bytes_to_read])?

        IF bytes_read == 0:
            BREAK  // End of file

        // Process the bytes
        FOR chunk IN buffer[0..bytes_read].chunks(bytes_per_line):
            print_line(offset, chunk, bytes_per_line, show_ascii)
            offset += chunk.length

        bytes_remaining -= bytes_read

    RETURN Ok(())

// Alternative: Using Take adapter
FUNCTION apply_length_limit(input_source, length):
    IF length IS Some(limit):
        RETURN input_source.take(limit)  // Wrap in Take adapter
    ELSE:
        RETURN input_source
```

### Phase 4 Implementation: Color Support

```
ENUM Color:
    BrightBlack  // Gray - for null bytes (0x00)
    Green        // Printable ASCII (0x20-0x7E)
    Cyan         // High bytes (0x80+)
    Yellow       // Control characters (0x01-0x1F)

FUNCTION colorize_hex_byte(byte, use_colors):
    hex_string = format!("{:02x}", byte)

    IF NOT use_colors:
        RETURN hex_string

    // Select color based on byte value
    color = MATCH byte:
        0x00 => Color::BrightBlack
        0x20..=0x7E => Color::Green
        0x80..=0xFF => Color::Cyan
        _ => Color::Yellow

    RETURN apply_ansi_color(hex_string, color)

FUNCTION apply_ansi_color(text, color):
    // ANSI escape codes for colors
    ansi_code = MATCH color:
        Color::BrightBlack => "\x1b[90m"  // Bright black (gray)
        Color::Green => "\x1b[32m"         // Green
        Color::Cyan => "\x1b[36m"          // Cyan
        Color::Yellow => "\x1b[33m"        // Yellow

    reset_code = "\x1b[0m"  // Reset to default

    RETURN ansi_code + text + reset_code

// Updated print_line with color support
FUNCTION print_line_colored(offset, data, bytes_per_line, show_ascii, use_colors):
    output = format_offset(offset) + ": "

    // Print hex bytes with colors
    FOR byte IN data:
        colored_hex = colorize_hex_byte(byte, use_colors)
        output += colored_hex + " "

    // Padding
    IF data.length < bytes_per_line:
        padding = " ".repeat((bytes_per_line - data.length) * 3)
        output += padding

    // ASCII column (also colorized)
    IF show_ascii:
        output += " |"
        FOR byte IN data:
            IF is_printable(byte):
                char = (byte as char)
                IF use_colors:
                    output += apply_ansi_color(char, Color::Green)
                ELSE:
                    output += char
            ELSE:
                output += "."
        output += "|"

    print(output)
```

### Phase 4 Implementation: Multiple Output Formats

```
FUNCTION format_line_by_type(offset, data, format, bytes_per_line, show_ascii, use_colors):
    MATCH format:
        OutputFormat::Canonical:
            RETURN format_canonical(offset, data, bytes_per_line, show_ascii, use_colors)

        OutputFormat::PlainHex:
            RETURN format_plain_hex(data, use_colors)

        OutputFormat::CArray:
            RETURN format_c_array(offset, data)

        OutputFormat::Uppercase:
            RETURN format_uppercase(offset, data, bytes_per_line, show_ascii, use_colors)

FUNCTION format_canonical(offset, data, bytes_per_line, show_ascii, use_colors):
    // Standard format: offset | hex | ASCII
    // This is the current implementation
    output = format!("{:08x}: ", offset)

    FOR byte IN data:
        hex = IF use_colors THEN colorize_hex_byte(byte, true)
              ELSE format!("{:02x}", byte)
        output += hex + " "

    IF data.length < bytes_per_line:
        output += " ".repeat((bytes_per_line - data.length) * 3)

    IF show_ascii:
        output += " |"
        FOR byte IN data:
            output += IF is_printable(byte) THEN (byte as char) ELSE '.'
        output += "|"

    RETURN output

FUNCTION format_plain_hex(data, use_colors):
    // Just hex bytes, no offset or ASCII
    // Example: 48 65 6c 6c 6f
    output = ""

    FOR i, byte IN data.enumerate():
        IF i > 0:
            output += " "

        hex = IF use_colors THEN colorize_hex_byte(byte, true)
              ELSE format!("{:02x}", byte)
        output += hex

    RETURN output

FUNCTION format_c_array(offset, data):
    // C-style array format
    // Example: 0x48, 0x65, 0x6c, 0x6c, 0x6f
    output = ""

    FOR i, byte IN data.enumerate():
        IF i > 0:
            output += ", "
        output += format!("0x{:02x}", byte)

    RETURN output

FUNCTION format_uppercase(offset, data, bytes_per_line, show_ascii, use_colors):
    // Same as canonical but with uppercase hex
    output = format!("{:08X}: ", offset)  // Uppercase offset

    FOR byte IN data:
        // Uppercase hex digits
        hex = IF use_colors THEN colorize_hex_byte_uppercase(byte, true)
              ELSE format!("{:02X}", byte)
        output += hex + " "

    IF data.length < bytes_per_line:
        output += " ".repeat((bytes_per_line - data.length) * 3)

    IF show_ascii:
        output += " |"
        FOR byte IN data:
            output += IF is_printable(byte) THEN (byte as char) ELSE '.'
        output += "|"

    RETURN output

FUNCTION colorize_hex_byte_uppercase(byte, use_colors):
    // Same as colorize_hex_byte but uppercase
    hex_string = format!("{:02X}", byte)  // Note: uppercase X

    IF NOT use_colors:
        RETURN hex_string

    color = MATCH byte:
        0x00 => Color::BrightBlack
        0x20..=0x7E => Color::Green
        0x80..=0xFF => Color::Cyan
        _ => Color::Yellow

    RETURN apply_ansi_color(hex_string, color)
```

### Phase 4 Implementation: Uppercase/Lowercase Options

```
// Add to HexDumper structure
STRUCTURE HexDumper:
    bytes_per_line: usize
    show_ascii: bool
    output_format: OutputFormat
    use_colors: bool
    use_uppercase: bool  // NEW: flag for uppercase hex
    offset: usize

FUNCTION format_hex_with_case(byte, uppercase):
    // Format byte as hex with specified case
    IF uppercase:
        RETURN format!("{:02X}", byte)  // Uppercase
    ELSE:
        RETURN format!("{:02x}", byte)  // Lowercase

FUNCTION format_offset_with_case(offset, uppercase):
    // Format offset with specified case
    IF uppercase:
        RETURN format!("{:08X}", offset)  // Uppercase
    ELSE:
        RETURN format!("{:08x}", offset)  // Lowercase

// Updated print_line with case support
FUNCTION print_line_with_case(offset, data, config):
    output = format_offset_with_case(offset, config.use_uppercase)
    output += ": "

    FOR byte IN data:
        hex = format_hex_with_case(byte, config.use_uppercase)
        IF config.use_colors:
            hex = apply_ansi_color(hex, get_color_for_byte(byte))
        output += hex + " "

    // ... rest of formatting
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
- [X] Set up project structure with Cargo
- [X] Implement basic file reading
- [X] Display simple hex output (no formatting)
- [X] Add offset column

### Phase 2: Standard Hex Dump Format
- [X] Implement canonical format (offset | hex | ASCII)
- [X] Add proper spacing and alignment
- [X] Handle partial lines at end of file
- [X] Add ASCII representation with non-printable character handling

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
