# Grep-lite

A lightweight grep implementation in Rust that searches for patterns in files or stdin using regular expressions.

## Features

- Search for regex patterns in files
- Read from stdin when no file is specified
- Fast and memory-efficient line-by-line processing
- Simple command-line interface

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/grep-lite`

## Usage

### Search in a file
```bash
./grep-lite "pattern" file.txt
```

### Search from stdin
```bash
cat file.txt | ./grep-lite "pattern"
echo "some text here" | ./grep-lite "text"
```

### Examples

```bash
# Find all lines containing "error" in a log file
./grep-lite "error" app.log

# Find email addresses
./grep-lite "\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b" contacts.txt

# Use with other commands
ps aux | ./grep-lite "python"
```

## Implementation Pseudocode

```
FUNCTION main():
    // Parse command-line arguments
    args = PARSE_ARGUMENTS()
    pattern = args.pattern
    input_file = args.input  // Optional
    
    // Compile the regex pattern
    regex = COMPILE_REGEX(pattern)
    
    // Determine input source
    IF input_file IS PROVIDED:
        file = OPEN_FILE(input_file)
        reader = CREATE_BUFFERED_READER(file)
        CALL process_line(reader, regex)
    ELSE:
        stdin = GET_STDIN()
        reader = LOCK_STDIN(stdin)
        CALL process_line(reader, regex)
    END IF
END FUNCTION


FUNCTION process_line(reader, regex):
    // Process each line from the input source
    FOR EACH line IN reader.lines():
        line_text = UNWRAP(line)
        
        // Check if the pattern matches
        match_result = regex.FIND(line_text)
        
        IF match_result IS FOUND:
            PRINT(line_text)
        END IF
    END FOR
END FUNCTION
```

## Algorithm Breakdown

1. **Argument Parsing**: Uses `clap` to parse command-line arguments
    - First positional argument: regex pattern (required)
    - Second positional argument: input file (optional)

2. **Regex Compilation**: Compiles the pattern into a regex object once for efficiency

3. **Input Source Selection**:
    - If a filename is provided: opens the file and wraps it in a buffered reader
    - If no filename: reads from stdin

4. **Line Processing**:
    - Reads input line-by-line (memory efficient)
    - Tests each line against the regex pattern
    - Prints lines that match

## Dependencies

Add these to your `Cargo.toml`:

```toml
[dependencies]
regex = "1.12.1"
clap = { version = "4.5", features = ["derive"] }
```

## Performance Considerations

- **Buffered I/O**: Uses `BufReader` for efficient reading
- **Line-by-line processing**: Doesn't load entire file into memory
- **Single regex compilation**: Pattern is compiled once and reused
- **Generic reader**: Abstracts over different input sources (file/stdin)

## Limitations

- Uses `unwrap()` for error handling (panics on errors)
- Prints entire matching lines (no highlighting or match extraction)
- No support for multiple files
- No inverse matching or case-insensitive flags


## Author

Onyekachukwu Nweke <nwekeejioforscheller@gmail.com>
