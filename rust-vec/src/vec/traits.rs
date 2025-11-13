use std::ops::{Deref, DerefMut};
use super::core::Vec;

// Clean up our mess when Vec goes out of scope
impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        // Drop all the elements by popping them
        // RawVec will handle deallocating the memory when it drops
        while let Some(_) = self.pop() {}
    }
}

// Let Vec pretend to be a slice for reading
impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr(), self.len)
        }
    }
}

// Let Vec pretend to be a slice for writing too
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr(), self.len)
        }
    }
}
