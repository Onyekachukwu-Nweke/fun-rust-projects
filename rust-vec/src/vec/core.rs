use super::rawvec::RawVec;

// Our custom Vec implementation
// Uses RawVec for memory management, separating allocation concerns from Vec logic
pub struct Vec<T> {
    buf: RawVec<T>,
    pub(super) len: usize,
}

// Make sure Vec can be sent across threads if T can
unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    // Creates a new empty Vec
    // We don't support zero-sized types because their layout is weird
    pub fn new() -> Self {
        Vec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    /// Returns the current capacity
    pub fn capacity(&self) -> usize {
        self.buf.cap()
    }

    /// Returns the number of elements in the vector
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    // Helper method to access the underlying pointer
    pub(super) fn ptr(&self) -> *mut T {
        self.buf.ptr().as_ptr()
    }

    // Helper method to access capacity
    pub(super) fn cap(&self) -> usize {
        self.buf.cap()
    }

    // Helper method to grow the buffer
    pub(super) fn grow(&mut self) {
        self.buf.grow();
    }
}
