#[derive(Debug, Clone)]
pub struct GapBuffer {
    buf: Vec<char>,
    gap_start: usize,
    gap_end: usize,
}

impl GapBuffer {
    /// Create an empty buffer with a gap of `capacity`.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut buf = vec!['\0'; capacity.max(8)];
        let len = buf.len();
        Self { buf, gap_start: 0, gap_end: len }
    }

    /// Create from initial text with extra gap capacity.
    pub fn from_str_with_capacity(s: &str, extra_gap: usize) -> Self {
        let text_len = s.chars().count();
        let total = text_len + extra_gap.max(8);
        let mut buf = vec!['\0'; total];
        // copy text before the gap
        for (i, c) in s.chars().enumerate() {
            buf[i] = c;
        }
        let gap_start = text_len;
        let gap_end = total;
        Self { buf, gap_start, gap_end }
    }

    pub fn from_str(s: &str) -> Self {
        Self::from_str_with_capacity(s, 64)
    }

    /// Current text length (excluding the gap).
    pub fn len(&self) -> usize {
        self.buf.len() - (self.gap_end - self.gap_start)
    }

    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// Number of slots in the gap.
    fn gap_size(&self) -> usize { self.gap_end - self.gap_start }

    fn ensure_gap(&mut self, need: usize) {
        if self.gap_size() >= need { return; }
        // grow by doubling-ish
        let current_text = self.len();
        let add = need.max(current_text.max(16));
        let new_len = self.buf.len() + add;

        let mut new_buf = vec!['\0'; new_len];

        // copy left part [0 .. gap_start)
        new_buf[..self.gap_start].copy_from_slice(&self.buf[..self.gap_start]);

        // copy right part [gap_end .. old_len) to end of new buffer
        let right_len = self.buf.len() - self.gap_end;
        let new_gap_end = new_len - right_len;
        new_buf[new_gap_end..].copy_from_slice(&self.buf[self.gap_end..]);

        self.buf = new_buf;
        self.gap_end = new_gap_end;
    }

    /// Insert a single character at the cursor.
    pub fn insert_char(&mut self, ch: char) {
        self.ensure_gap(1);
        self.buf[self.gap_start] = ch;
        self.gap_start += 1;
    }

    /// Insert a &str at the cursor.
    pub fn insert_str(&mut self, s: &str) {
        self.ensure_gap(s.chars().count());
        for ch in s.chars() {
            self.buf[self.gap_start] = ch;
            self.gap_start += 1;
        }
    }

    /// Delete one character to the right of the cursor (Delete key).
    pub fn delete(&mut self) -> bool {
        if self.gap_end < self.buf.len() {
            self.gap_end += 1;
            true
        } else {
            false
        }
    }

    /// Backspace one character to the left of the cursor.
    pub fn backspace(&mut self) -> bool {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            true
        } else {
            false
        }
    }

    /// Move cursor left by 1 (shifts one char from left to right side of gap).
    pub fn move_left(&mut self) -> bool {
        if self.gap_start > 0 {
            self.gap_start -= 1;
            self.gap_end -= 1;
            self.buf[self.gap_end] = self.buf[self.gap_start];
            true
        } else {
            false
        }
    }

    /// Move cursor right by 1 (shifts one char from right to left side of gap).
    pub fn move_right(&mut self) -> bool {
        if self.gap_end < self.buf.len() {
            self.buf[self.gap_start] = self.buf[self.gap_end];
            self.gap_start += 1;
            self.gap_end += 1;
            true
        } else {
            false
        }
    }

    /// Collect full text (skips the gap).
    pub fn text(&self) -> String {
        let mut out = String::with_capacity(self.len());
        for (i, &c) in self.buf.iter().enumerate() {
            if i >= self.gap_start && i < self.gap_end { continue; }
            if c != '\0' { out.push(c); }
        }
        out
    }

    /// Current cursor offset in *logical text* (index ignoring the gap).
    pub fn cursor_offset(&self) -> usize {
        self.gap_start
    }
}