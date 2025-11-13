use std::ptr;
use super::core::Vec;

impl<T> Vec<T> {
    // Add an element to the end
    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.grow();
        }

        unsafe {
            // Write to uninitialized memory at the end
            ptr::write(self.ptr().add(self.len), elem);
        }

        // We'll run out of memory before this overflows
        self.len += 1;
    }

    // Remove and return the last element
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                // Read the value out without dropping it
                Some(ptr::read(self.ptr().add(self.len)))
            }
        }
    }

    // Insert an element at a specific position
    // Index can be anywhere from 0 to len (inclusive) - inserting at len is like push
    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.len == self.cap() {
            self.grow();
        }

        unsafe {
            // Shift everything after index one spot to the right
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            // Write the new element
            ptr::write(self.ptr().add(index), elem);
        }

        self.len += 1;
    }

    // Remove and return an element at a specific position
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            // Read out the value we're removing
            let result = ptr::read(self.ptr().add(index));
            // Shift everything after it one spot to the left
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}
