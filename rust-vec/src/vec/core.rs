use std::ptr::NonNull;
use std::mem;

// Our custom Vec implementation
// Uses raw pointers and manual memory management for learning purposes
pub struct Vec<T> {
    pub(super) ptr: NonNull<T>,
    pub(super) cap: usize,
    pub(super) len: usize,
}

// Make sure Vec can be sent across threads if T can
unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    // Creates a new empty Vec
    // We don't support zero-sized types because their layout is weird
    pub fn new() -> Self {
        assert_ne!(mem::size_of::<T>(), 0, "zero sized types are not supported");
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }
}
