# Wordie

A minimal text editor implementation in Rust, built around the gap buffer data structure. This project demonstrates core text editing primitives and serves as an educational reference for understanding how text editors manage document state efficiently.

## Overview

Wordie implements a gap buffer-based text editor with cursor tracking, line-based navigation, and file I/O. The gap buffer is a fundamental data structure used in production text editors like Emacs and historically in many word processors due to its efficient handling of local editing operations.

## Technical Concepts

### Gap Buffer

A **gap buffer** is a dynamic array that maintains a contiguous "gap" (empty space) at the cursor position. Text before the cursor sits at the start of the array, text after the cursor sits at the end, and the middle contains unused capacity.

**Structure:**
```
[t][e][x][t][ ][ ][ ][m][o][r][e]
            ^gap^
            cursor is here
```

**Key Properties:**
- Insertions at the cursor are O(1) - just fill the gap and advance
- Deletions at the cursor are O(1) - expand the gap
- Cursor movements are O(1) per character - shift one character across the gap boundary
- Random access requires O(n) traversal in worst case
- Optimal for typical editing patterns where edits cluster in one location

**Why Gap Buffers Matter:**

Text editors need to handle frequent insertions and deletions at the cursor position. Alternative data structures have tradeoffs:
- **Simple arrays**: Insertions/deletions are O(n) due to shifting
- **Linked lists**: O(1) insertion but poor cache locality and memory overhead
- **Rope structures**: More complex, better for large documents with edits across multiple locations
- **Gap buffers**: Sweet spot for typical editing - O(1) local edits with good cache locality

### Implementation Details

The implementation consists of three layers:

#### 1. GapBuffer (gap_buffer.rs)

The core data structure managing the buffer and gap invariants.

**Pseudocode for key operations:**

```
STRUCT GapBuffer:
    buf: array of characters
    gap_start: index where gap begins
    gap_end: index where gap ends

FUNCTION insert_char(ch):
    if gap_size == 0:
        expand_buffer()

    buf[gap_start] = ch
    gap_start += 1

FUNCTION move_left():
    if gap_start == 0:
        return false

    // Shift character from left side to right side of gap
    gap_start -= 1
    gap_end -= 1
    buf[gap_end] = buf[gap_start]
    return true

FUNCTION move_right():
    if gap_end == buf.length:
        return false

    // Shift character from right side to left side of gap
    buf[gap_start] = buf[gap_end]
    gap_start += 1
    gap_end += 1
    return true

FUNCTION text():
    result = empty string
    for i from 0 to buf.length:
        if i < gap_start OR i >= gap_end:
            result.append(buf[i])
    return result
```

**Buffer Expansion:**

When the gap fills up, the buffer grows:
1. Allocate new buffer with additional capacity (doubling strategy)
2. Copy prefix `[0..gap_start)` to new buffer
3. Copy suffix `[gap_end..old_len)` to end of new buffer
4. Gap now occupies the middle

```
FUNCTION ensure_gap(needed_space):
    if gap_size >= needed_space:
        return

    current_text_len = buf.length - (gap_end - gap_start)
    additional = max(needed_space, current_text_len, 16)
    new_length = buf.length + additional

    new_buf = allocate(new_length)

    // Copy prefix
    copy buf[0..gap_start] to new_buf[0..gap_start]

    // Copy suffix
    right_length = buf.length - gap_end
    new_gap_end = new_length - right_length
    copy buf[gap_end..] to new_buf[new_gap_end..]

    buf = new_buf
    gap_end = new_gap_end
```

#### 2. Editor (editor.rs)

Higher-level abstraction that adds line/column semantics on top of the gap buffer.

**Line/Column Tracking:**

The editor computes line and column positions by scanning the text for newlines. This is done on-demand rather than maintaining incremental state.

```
FUNCTION cursor_pos():
    text = gap_buffer.text()
    offset = gap_buffer.cursor_offset()

    line = 0
    col = 0

    for i from 0 to offset:
        if text[i] == '\n':
            line += 1
            col = 0
        else:
            col += 1

    return (line, col)
```

**Vertical Navigation:**

Moving up/down requires mapping (line, col) coordinates to a linear offset:

```
FUNCTION move_to(target_line, target_col):
    text = gap_buffer.text()

    // Build array of line start positions
    line_starts = [0]
    for i, char in text:
        if char == '\n':
            line_starts.append(i + 1)

    // Clamp line to valid range
    line = min(target_line, line_starts.length - 1)

    // Find line boundaries
    line_start = line_starts[line]
    if line + 1 < line_starts.length:
        line_end = line_starts[line + 1] - 1
    else:
        line_end = text.length

    // Clamp column to line length
    line_length = line_end - line_start
    col = min(target_col, line_length)

    target_offset = line_start + col

    // Move gap to target by stepping
    while cursor_offset < target_offset:
        move_right()
    while cursor_offset > target_offset:
        move_left()
```

#### 3. I/O Layer (io.rs)

Simple file operations for loading and saving editor content.

## Architecture

```
┌─────────────────────────────────────────┐
│              Application                │
│         (main.rs - demo mode)           │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│         Editor Interface                │
│   • Line/column abstraction             │
│   • Vertical navigation (up/down)       │
│   • Cursor position queries             │
└────────────────┬────────────────────────┘
                 │
┌────────────────▼────────────────────────┐
│          Gap Buffer Engine              │
│   • Character insertion/deletion        │
│   • Horizontal cursor movement          │
│   • Buffer management & resizing        │
│   • Text reconstruction                 │
└─────────────────────────────────────────┘
```

**Design Rationale:**

- **Separation of concerns**: Gap buffer handles raw buffer mechanics; editor adds semantic operations
- **Minimal dependencies**: Zero external crates - pure Rust standard library
- **Educational clarity**: Code favors readability over micro-optimizations
- **Testability**: Each layer can be tested independently

## Usage

**Demo mode:**
```bash
cargo run -- demo
```

This runs a simple demonstration that:
1. Creates an editor with "Hello World"
2. Moves cursor left twice (before 'r')
3. Inserts "! "
4. Saves result to `out.txt`
5. Prints: "Hello Wo! rld"

**As a library:**
```rust
use wordie::Editor;

let mut ed = Editor::from_str("Hello\nWorld");
ed.move_down();           // Move to line 1
ed.move_right();          // Move right on "World"
ed.insert_char('!');      // Insert '!'
println!("{}", ed.text());
```

## Project Structure

```
wordie/
├── src/
│   ├── gap_buffer.rs    # Core gap buffer implementation
│   ├── editor.rs        # Editor with line/column support
│   ├── io.rs            # File I/O utilities
│   ├── lib.rs           # Library exports
│   └── main.rs          # Demo application
├── Cargo.toml
└── README.md
```

## Technical References

### Gap Buffer Research and Documentation

1. **"Data Structures for Text Sequences"** - Charles Crowley
   - [University of New Mexico Technical Report](https://www.cs.unm.edu/~crowley/papers/sds.pdf)
   - Comprehensive comparison of text editor data structures including gap buffers

2. **GNU Emacs Internals Manual**
   - [Emacs Buffer Representation](https://www.gnu.org/software/emacs/manual/html_node/elisp/Buffer-Internals.html)
   - Real-world production implementation of gap buffers

3. **"Craft of Text Editing"** - Craig A. Finseth
   - Classic reference on text editor implementation
   - Available at: http://www.finseth.com/craft/

4. **"Data Structures and Algorithms for Big Databases"** - Tokutek (MongoDB)
   - Section on gap buffer vs. other text buffer structures
   - Discusses when gap buffers are optimal vs rope/piece table

### Algorithm Complexity

| Operation | Time Complexity | Notes |
|-----------|----------------|-------|
| Insert at cursor | O(1) amortized | O(n) when buffer needs expansion |
| Delete at cursor | O(1) | |
| Move cursor left/right | O(1) | Per-character basis |
| Text reconstruction | O(n) | n = document length |
| Cursor position query | O(n) | Scans for newlines |
| Move to line/col | O(n) | Builds line index + cursor movements |

### Memory Characteristics

- **Space overhead**: O(1) extra space for gap (typically 64+ chars)
- **Fragmentation**: None - single contiguous allocation
- **Cache behavior**: Excellent for sequential access patterns
- **Growth strategy**: Exponential (doubling) to amortize allocations

## Limitations and Future Work

**Current Limitations:**
- Cursor position/line operations are O(n) - acceptable for small files
- No incremental line indexing (regenerates line map on each query)
- UTF-8 is handled via Rust's `char` type but no grapheme cluster awareness
- No undo/redo system

**Potential Enhancements:**
- Add incremental line indexing for O(1) line/column queries
- Implement piece table for better undo/redo support
- Add rope structure option for very large files (>1MB)
- Unicode grapheme cluster support for proper cursor movement
- Syntax highlighting hooks
- Multiple cursor support

## Building

```bash
# Build
cargo build

# Run tests
cargo test

# Run demo
cargo run -- demo

# Build optimized release
cargo build --release
```

## License

See project metadata for license information.

## Author

Onyekachukwu Nweke (nwekeejioforscheller@gmail.com)

## Learning Resources

If you're studying text editor implementation, consider exploring:

1. **Emacs source code**: `buffer.c` and `insdel.c` show production gap buffer usage
2. **Xi Editor** (Rust): Modern rope-based editor for comparison
3. **Textadept**: Scintilla-based editor with accessible C implementation
4. **Build Your Own Text Editor** tutorial (kilo): https://viewsourcecode.org/snaptoken/kilo/

This project demonstrates fundamental concepts. Production editors add layers of complexity for performance, undo systems, syntax highlighting, and multi-buffer management.
