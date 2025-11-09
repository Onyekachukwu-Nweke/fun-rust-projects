use std::ops::{Deref, DerefMut};
use std::alloc::{self, Layout};
use super::core::Vec;

// Clean up our mess when Vec goes out of scope
impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // Drop all the elements by popping them
            while let Some(_) = self.pop() {}

            // Deallocate the memory
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

// Let Vec pretend to be a slice for reading
impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe {
            std::slice::from_raw_parts(self.ptr.as_ptr(), self.len)
        }
    }
}

// Let Vec pretend to be a slice for writing too
impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len)
        }
    }
}
