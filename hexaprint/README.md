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
**File: `src/main.rs`**

```rust
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
            RETURN BufReader::new(stdin.lock())

// Updated main flow with stdin support
// Modify the existing main() function in src/main.rs
FUNCTION main():
    args = parse_arguments()

    // Validate arguments (existing code)
    IF args.output.is_some() AND NOT args.binary:
        eprintln!("--output can only be used with --binary")
        exit(1)

    // CHANGE: Remove the error when input_file is None
    // Instead, get input from file or stdin
    input_source = MATCH args.input_file:
        Some(path):
            file = File::open(&path).unwrap_or_else(|e| {
                eprintln!("Error opening file '{}': {}", path, e)
                exit(1)
            })
            BufReader::new(file)
        None:
            // Read from stdin
            BufReader::new(io::stdin().lock())

    // Create and run dumper
    IF args.binary:
        // ... existing binary mode logic
    ELSE:
        // Hex dump mode
        dumper = HexDumper::new(
            args.bytes_per_line,
            args.show_ascii,
            args.output_format,
            args.use_colors,
        )
        dumper.dump(input_source).unwrap()
```

### Phase 3 Implementation: Skip and Length Options

**Files to modify:**
1. `src/main.rs` - Add CLI arguments for skip and length
2. `src/dumper.rs` - Modify `dump()` method to support length limit

**Step 1: Add CLI arguments in `src/main.rs`**

```rust
// Add to Args struct in src/main.rs
#[derive(Parser)]
struct Args {
    // ... existing fields ...

    /// Number of bytes to display
    #[arg(short = 'n', long)]
    length: Option<usize>,

    /// Skip N bytes from start
    #[arg(short = 's', long)]
    skip: Option<usize>,
}
```

**Step 2: Apply skip in `src/main.rs` main() function**

```rust
FUNCTION main():
    // ... args parsing and input_source setup ...

    // Apply skip if specified
    IF args.skip IS Some(skip_bytes):
        use std::io::Read;
        let mut discard = vec![0; skip_bytes];
        IF input_source.read_exact(&mut discard).is_err():
            eprintln!("Error: Cannot skip {} bytes (file too short)", skip_bytes)
            exit(1)

    // If length is specified, wrap input in Take adapter
    let limited_input = MATCH args.length:
        Some(limit) => Box::new(input_source.take(limit as u64)),
        None => Box::new(input_source)

    // Create dumper with offset adjusted for skip
    dumper = HexDumper::new(...)
    dumper.set_offset(args.skip.unwrap_or(0))
    dumper.dump(limited_input).unwrap()
```

**Step 3: Add `set_offset()` method in `src/dumper.rs`**

```rust
// In src/dumper.rs, add to impl HexDumper:
FUNCTION set_offset(offset: usize):
    self.offset = offset
```

**Alternative Approach: Handle skip/length in dumper.rs**

```rust
// Modify dump() in src/dumper.rs to accept skip and length
FUNCTION dump_with_limits(input, skip: Option<usize>, length: Option<usize>):
    // Skip bytes if needed
    IF skip IS Some(n) AND n > 0:
        discard = vec![0; n]
        input.read_exact(&mut discard)?
        self.offset = n

    // Determine how many bytes to read total
    bytes_remaining = length.unwrap_or(usize::MAX)
    offset = self.offset
    buffer = vec![0; BUFFER_SIZE]

    LOOP:
        // Don't read more than bytes_remaining
        read_size = min(buffer.len(), bytes_remaining)
        IF read_size == 0:
            BREAK

        bytes_read = read_chunk(input, &mut buffer[0..read_size])?
        IF bytes_read == 0:
            BREAK

        FOR chunk IN buffer[0..bytes_read].chunks(self.bytes_per_line):
            print_line(offset, chunk, self.bytes_per_line, self.show_ascii)
            offset += chunk.len()

        bytes_remaining -= bytes_read

    Ok(())
```

### Phase 4 Implementation: Color Support

**New file to create: `src/color.rs`**

This module will handle all color-related functionality.

```rust
// src/color.rs - NEW FILE

use std::fmt;

ENUM Color:
    BrightBlack  // Gray - for null bytes (0x00)
    Green        // Printable ASCII (0x20-0x7E)
    Cyan         // High bytes (0x80+)
    Yellow       // Control characters (0x01-0x1F)

FUNCTION get_color_for_byte(byte: u8) -> Color:
    // Select color based on byte value
    MATCH byte:
        0x00 => Color::BrightBlack
        0x20..=0x7E => Color::Green
        0x80..=0xFF => Color::Cyan
        _ => Color::Yellow

FUNCTION colorize(text: &str, color: Color, use_colors: bool) -> String:
    IF NOT use_colors:
        RETURN text.to_string()

    ansi_code = MATCH color:
        Color::BrightBlack => "\x1b[90m"  // Bright black (gray)
        Color::Green => "\x1b[32m"         // Green
        Color::Cyan => "\x1b[36m"          // Cyan
        Color::Yellow => "\x1b[33m"        // Yellow

    reset_code = "\x1b[0m"  // Reset to default
    RETURN format!("{}{}{}", ansi_code, text, reset_code)

FUNCTION colorize_byte(byte: u8, use_colors: bool) -> String:
    // Format byte as hex and apply color
    hex_string = format!("{:02x}", byte)

    IF NOT use_colors:
        RETURN hex_string

    color = get_color_for_byte(byte)
    RETURN colorize(&hex_string, color, use_colors)
```

**Modify `src/main.rs` to declare the module:**

```rust
// Add at the top of src/main.rs with other mod declarations
mod color;
```

**Modify `src/formatter.rs` to use colors:**

```rust
// Update imports in src/formatter.rs
use crate::color::{colorize_byte, get_color_for_byte, colorize};

// Modify print_line signature to accept use_colors parameter
FUNCTION print_line(offset, data, bytes_per_line, show_ascii, use_colors):
    output = format_offset(offset)
    output += ": "

    // Print hex bytes with colors
    FOR byte IN data:
        colored_hex = colorize_byte(*byte, use_colors)
        output += &colored_hex
        output += " "

    // Padding for incomplete lines
    IF data.len() < bytes_per_line:
        padding_spaces = (bytes_per_line - data.len()) * 3
        output += &" ".repeat(padding_spaces)

    // ASCII column (also colorized)
    IF show_ascii:
        output += " |"
        FOR byte IN data:
            IF is_printable(*byte):
                char_str = (*byte as char).to_string()
                IF use_colors:
                    output += &colorize(&char_str, get_color_for_byte(*byte), true)
                ELSE:
                    output += &char_str
            ELSE:
                output += "."
        output += "|"

    println!("{}", output)
```

**Modify `src/dumper.rs` to pass use_colors:**

```rust
// Update the dump() method in src/dumper.rs
FUNCTION dump(input):
    // ... existing code ...

    FOR chunk IN buffer[..bytes_read].chunks(self.bytes_per_line):
        print_line(
            offset,
            chunk,
            self.bytes_per_line,
            self.show_ascii,
            self.use_colors,  // ADD THIS PARAMETER
        )
        offset += chunk.len()
```

### Phase 4 Implementation: Multiple Output Formats

**Files to modify:**
1. `src/formatter.rs` - Add format-specific functions
2. `src/dumper.rs` - Use output_format in dump() method

**Option A: Modify `src/formatter.rs` with format functions**

```rust
// Add these functions to src/formatter.rs

FUNCTION format_line_by_type(offset, data, format, bytes_per_line, show_ascii, use_colors):
    MATCH format:
        OutputFormat::Canonical:
            RETURN format_canonical(offset, data, bytes_per_line, show_ascii, use_colors)

        OutputFormat::PlainHex:
            RETURN format_plain_hex(data, use_colors)

        OutputFormat::CArray:
            RETURN format_c_array(data)

        OutputFormat::Uppercase:
            RETURN format_uppercase(offset, data, bytes_per_line, show_ascii, use_colors)

FUNCTION format_canonical(offset, data, bytes_per_line, show_ascii, use_colors):
    // This is basically the current print_line implementation
    // Delegate to existing print_line function
    print_line(offset, data, bytes_per_line, show_ascii, use_colors)

FUNCTION format_plain_hex(offset, data, use_colors):
    // Just hex bytes, no offset or ASCII
    // Example: 48 65 6c 6c 6f
    let mut output = String::new()

    FOR (i, byte) IN data.iter().enumerate():
        IF i > 0:
            output.push(' ')

        hex = IF use_colors:
            colorize_byte(*byte, use_colors)
        ELSE:
            format!("{:02x}", *byte)

        output.push_str(&hex)

    println!("{}", output)

FUNCTION format_c_array(offset, data):
    // C-style array format
    // Example: 0x48, 0x65, 0x6c, 0x6c, 0x6f,
    let mut output = String::new()

    FOR (i, byte) IN data.iter().enumerate():
        IF i > 0:
            output.push_str(", ")
        output.push_str(&format!("0x{:02x}", byte))

    output.push(',')  // Trailing comma for C array
    println!("{}", output)

FUNCTION format_uppercase(offset, data, bytes_per_line, show_ascii, use_colors):
    // Same as canonical but with uppercase hex
    let mut output = format!("{:08X}: ", offset)  // Uppercase offset

    FOR byte IN data:
        hex = format!("{:02X}", *byte)  // Uppercase hex
        IF use_colors:
            let color = get_color_for_byte(*byte)
            output.push_str(&colorize(&hex, color, true))
        ELSE:
            output.push_str(&hex)
        output.push(' ')

    // Padding
    IF data.len() < bytes_per_line:
        let padding = " ".repeat((bytes_per_line - data.len()) * 3)
        output.push_str(&padding)

    // ASCII column
    IF show_ascii:
        output.push_str(" |")
        FOR byte IN data:
            IF is_printable(*byte):
                output.push(*byte as char)
            ELSE:
                output.push('.')
        output.push('|')

    println!("{}", output)
```

**Modify `src/dumper.rs` to use format functions:**

```rust
// Update dump() method in src/dumper.rs
use crate::formatter::{format_line_by_type};
use crate::dumper::OutputFormat;

FUNCTION dump(input):
    // ... existing code ...

    FOR chunk IN buffer[..bytes_read].chunks(self.bytes_per_line):
        // Use format dispatcher instead of direct print_line
        format_line_by_type(
            offset,
            chunk,
            self.output_format,  // USE THIS FIELD
            self.bytes_per_line,
            self.show_ascii,
            self.use_colors,
        )
        offset += chunk.len()
```

**Alternative: Create separate formatter module**

You could also create `src/formats.rs` to keep all format-specific code separate:

```rust
// src/formats.rs - NEW FILE (optional)
use crate::dumper::OutputFormat;
use crate::formatter::{format_offset, format_hex, is_printable};
use crate::color::{colorize_byte, get_color_for_byte, colorize};

pub fn format_and_print(
    offset: usize,
    data: &[u8],
    format: OutputFormat,
    bytes_per_line: usize,
    show_ascii: bool,
    use_colors: bool,
) {
    match format {
        OutputFormat::Canonical => format_canonical(...),
        OutputFormat::PlainHex => format_plain_hex(...),
        OutputFormat::CArray => format_c_array(...),
        OutputFormat::Uppercase => format_uppercase(...),
    }
}

// ... format functions here ...
```

### Phase 4 Implementation: Uppercase/Lowercase Options

**Note:** This feature is already partially implemented via `OutputFormat::Uppercase`.
However, if you want a separate `--uppercase` flag that works with all formats:

**Modify `src/formatter.rs`:**

```rust
// Add helper functions to src/formatter.rs

pub fn format_hex_with_case(byte: u8, uppercase: bool) -> String:
    IF uppercase:
        format!("{:02X}", byte)  // Uppercase
    ELSE:
        format!("{:02x}", byte)  // Lowercase

pub fn format_offset_with_case(offset: usize, uppercase: bool) -> String:
    IF uppercase:
        format!("{:08X}", offset)  // Uppercase
    ELSE:
        format!("{:08x}", offset)  // Lowercase
```

**Alternative: Use OutputFormat::Uppercase**

The cleaner approach is to use the existing `OutputFormat::Uppercase` enum variant,
which is already defined. No additional code needed - just use `-o uppercase` flag.

---

## Summary: Files to Create/Modify for Phase 3 & 4

### New Files to Create:

1. **`src/color.rs`** (Phase 4 - Color Support)
   - `Color` enum
   - `get_color_for_byte()` function
   - `colorize()` function
   - `colorize_byte()` function

2. **`src/formats.rs`** (Optional - Phase 4 - Multiple Formats)
   - `format_and_print()` dispatcher
   - `format_canonical()`
   - `format_plain_hex()`
   - `format_c_array()`
   - `format_uppercase()`

### Existing Files to Modify:

1. **`src/main.rs`**
   - **Phase 3**:
     - Remove error when `input_file` is None
     - Add stdin support
     - Add `length` and `skip` CLI arguments
     - Apply skip/length logic before calling dumper
   - **Phase 4**:
     - Add `mod color;` declaration
     - Add `mod formats;` (if using separate formats module)

2. **`src/dumper.rs`**
   - **Phase 3**:
     - Add `set_offset()` method (optional)
     - OR modify `dump()` to handle skip/length
   - **Phase 4**:
     - Pass `use_colors` to print_line
     - Pass `output_format` to format dispatcher
     - Use `format_line_by_type()` instead of direct `print_line()`

3. **`src/formatter.rs`**
   - **Phase 4**:
     - Update `print_line()` signature to accept `use_colors: bool`
     - Import and use color functions
     - Add `format_line_by_type()` dispatcher (if not in separate module)
     - Add format-specific functions (or move to `formats.rs`)
     - Add `format_hex_with_case()` and `format_offset_with_case()` (optional)

### File Structure After Phase 3 & 4:

```
hexaprint/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs          // CLI args, stdin support, skip/length
    ├── dumper.rs        // HexDumper struct, dump() method
    ├── formatter.rs     // Line formatting, helpers
    ├── color.rs         // NEW: Color support (ANSI codes)
    └── formats.rs       // NEW (optional): Output format implementations
```

---

## Quick Implementation Checklist

### Phase 3 Remaining Tasks:

**1. stdin Support (src/main.rs):**
- [ ] Change lines 61-67 from error exit to stdin fallback
- [ ] Use `io::stdin().lock()` when `args.input_file` is `None`

**2. Skip Option (src/main.rs):**
- [ ] Add `skip: Option<usize>` to Args struct
- [ ] Read and discard skip bytes before calling dumper
- [ ] Pass skip value to dumper for offset display

**3. Length Option (src/main.rs):**
- [ ] Add `length: Option<usize>` to Args struct
- [ ] Wrap input in `.take(length)` adapter

### Phase 4 Remaining Tasks:

**1. Color Support:**
- [ ] Create `src/color.rs` with Color enum and colorize functions
- [ ] Add `mod color;` to `src/main.rs`
- [ ] Update `print_line()` in `src/formatter.rs` to accept `use_colors: bool`
- [ ] Pass `self.use_colors` from `src/dumper.rs` to `print_line()`
- [ ] Apply colors to hex bytes and ASCII characters

**2. Output Formats:**
- [ ] Add format functions to `src/formatter.rs` (or new `src/formats.rs`)
- [ ] Create `format_line_by_type()` dispatcher
- [ ] Implement `format_plain_hex()`, `format_c_array()`, `format_uppercase()`
- [ ] Call dispatcher from `src/dumper.rs` instead of `print_line()`
- [ ] Pass `self.output_format` to dispatcher

**3. Uppercase (Already exists via OutputFormat::Uppercase):**
- [ ] No additional work needed - users can use `-o uppercase` flag

---

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
- [X] Add command-line argument parsing (clap is integrated)
- [ ] Support reading from stdin (currently requires file)
- [X] Implement configurable bytes per line (--bytes-per-line flag exists)
- [ ] Add skip and length options (CLI args don't exist yet)

### Phase 4: Enhanced Output
- [ ] Add color support (field exists but not used in formatter)
- [ ] Implement multiple output formats (enum exists but not used in formatter)
- [ ] Add uppercase/lowercase hex options (OutputFormat::Uppercase exists)
- [X] Improve error messages (good error handling exists)

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
