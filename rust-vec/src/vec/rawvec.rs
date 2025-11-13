use std::alloc::{self, Layout};
use std::ptr::NonNull;
use std::mem;

/// RawVec handles the raw memory allocation for Vec
/// Separates memory management concerns from Vec's logic
pub struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

// Make sure RawVec can be sent across threads if T can
unsafe impl<T: Send> Send for RawVec<T> {}
unsafe impl<T: Sync> Sync for RawVec<T> {}

impl<T> RawVec<T> {
    /// Creates a new RawVec with zero capacity
    /// We don't support zero-sized types because their layout is weird
    pub fn new() -> Self {
        assert_ne!(mem::size_of::<T>(), 0, "zero sized types are not supported");
        RawVec {
            ptr: NonNull::dangling(),
            cap: 0,
        }
    }

    /// Returns the current capacity
    pub fn cap(&self) -> usize {
        self.cap
    }

    /// Returns the raw pointer
    pub fn ptr(&self) -> NonNull<T> {
        self.ptr
    }

    /// Doubles the capacity when we run out of space
    /// Starts at 1 if we're currently empty
    pub fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // Double it - can't overflow since cap is always <= isize::MAX
            let new_cap = 2 * self.cap;

            // This unwrap is safe because if we already had an allocation,
            // doubling it won't suddenly become too large
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Sanity check - make sure we're not trying to allocate something massive
        assert!(new_layout.size() <= isize::MAX as usize, "Allocation too large");

        let new_ptr = if self.cap == 0 {
            // First allocation - just allocate
            unsafe { alloc::alloc(new_layout) }
        } else {
            // We already have memory, so resize it
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, new_ptr will be null and we abort
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // Deallocate the memory
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}
