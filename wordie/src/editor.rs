use crate::GapBuffer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPos {
    pub line: usize,
    pub col: usize,
}

#[derive(Debug, Default)]
pub struct Editor {
    gb: GapBuffer,
}

impl Editor {
    pub fn new() -> Self { Self { gb: GapBuffer::with_capacity(64) } }
    pub fn from_str(s: &str) -> Self { Self { gb: GapBuffer::from_str(s) } }

    pub fn insert_char(&mut self, c: char) { self.gb.insert_char(c); }
    pub fn insert_str(&mut self, s: &str) { self.gb.insert_str(s); }
    pub fn delete(&mut self) -> bool { self.gb.delete() }
    pub fn backspace(&mut self) -> bool { self.gb.backspace() }
    pub fn move_left(&mut self) -> bool { self.gb.move_left() }
    pub fn move_right(&mut self) -> bool { self.gb.move_right() }

    pub fn text(&self) -> String { self.gb.text() }

    /// Return the nth 0-based line as String.
    pub fn line(&self, n: usize) -> Option<String> {
        self.gb
            .text()
            .lines()
            .map(|s| s.to_string())
            .nth(n)
    }

    /// Compute current (line, col) from cursor offset.
    pub fn cursor_pos(&self) -> CursorPos {
        let text = self.gb.text();
        let cur = self.gb.cursor_offset();

        let mut idx = 0usize;
        let mut line = 0usize;
        let mut col = 0usize;

        for ch in text.chars() {
            if idx == cur { break; }
            if ch == '\n' { line += 1; col = 0; } else { col += 1; }
            idx += 1;
        }
        CursorPos { line, col }
    }

    /// Move cursor to (line, col), clamped.
    pub fn move_to(&mut self, line: usize, col: usize) {
        let text = self.gb.text();
        // Build line starts
        let mut starts = vec![0usize];
        for (i, ch) in text.chars().enumerate() {
            if ch == '\n' { starts.push(i + 1); }
        }
        // Compute target
        let line = line.min(starts.len().saturating_sub(1));
        let line_start = starts[line];
        let line_end = if line + 1 < starts.len() { starts[line + 1] - 1 } else { text.chars().count() };
        let line_len = line_end.saturating_sub(line_start);
        let target = line_start + col.min(line_len);

        // Move the gap to target by stepping left/right.
        while self.gb.cursor_offset() < target { self.gb.move_right(); }
        while self.gb.cursor_offset() > target { self.gb.move_left(); }
    }

    pub fn move_up(&mut self) {
        let pos = self.cursor_pos();
        let line = pos.line.saturating_sub(1);
        self.move_to(line, pos.col);
    }

    pub fn move_down(&mut self) {
        let pos = self.cursor_pos();
        self.move_to(pos.line + 1, pos.col);
    }
}