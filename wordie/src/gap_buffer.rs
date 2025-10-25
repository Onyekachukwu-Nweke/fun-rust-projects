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
}