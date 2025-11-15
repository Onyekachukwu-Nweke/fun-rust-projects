use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

/// A memory pool that pre-allocates a fixed number of blocks
/// of a specific size, allowing fast allocation and deallocation.
///
/// Benefits:
/// - O(1) allocation and deallocation
/// - No fragmentation within the pool
/// - Cache-friendly due to contiguous memory layout
pub struct MemoryPool {
    // Pointer to the start of our memory block
    memory: NonNull<u8>,
    // Size of each individual block
    block_size: usize,
    // Total number of blocks in the pool
    total_blocks: usize,
    // Head of the free list (index of next free block)
    free_list_head: Option<usize>,
    // Track how many blocks are currently in use
    blocks_in_use: usize,
}

impl MemoryPool {
    /// Creates a new memory pool with the specified block size and count.
    ///
    /// # Panics
    /// Panics if block_size is 0 or total_blocks is 0
    pub fn new(block_size: usize, total_blocks: usize) -> Self {
        assert!(block_size > 0, "Block size must be greater than 0");
        assert!(total_blocks > 0, "Total blocks must be greater than 0");

        // Ensure block size is at least large enough to store a pointer (for the free list)
        let block_size = block_size.max(std::mem::size_of::<usize>());

        // Allocate the entire memory pool in one go
        let layout = Layout::from_size_align(block_size * total_blocks, 8)
            .expect("Failed to create layout");

        let memory = unsafe {
            let ptr = alloc(layout);
            NonNull::new(ptr).expect("Failed to allocate memory pool")
        };

        let mut pool = MemoryPool {
            memory,
            block_size,
            total_blocks,
            free_list_head: Some(0),
            blocks_in_use: 0,
        };

        // Initialize the free list: each block points to the next
        pool.initialize_free_list();
        pool
    }

    /// Sets up the free list by making each block point to the next one.
    /// The last block points to None (end of list).
    fn initialize_free_list(&mut self) {
        unsafe {
            for i in 0..self.total_blocks {
                let block_ptr = self.get_block_ptr(i);
                let next_index = if i + 1 < self.total_blocks {
                    i + 1
                } else {
                    // Use usize::MAX to represent None
                    usize::MAX
                };

                // Store the next index at the beginning of this block
                *(block_ptr as *mut usize) = next_index;
            }
        }
    }

    /// Returns a raw pointer to the block at the given index
    unsafe fn get_block_ptr(&self, index: usize) -> *mut u8 {
        self.memory.as_ptr().add(index * self.block_size)
    }

    /// Allocates a block from the pool.
    /// Returns None if the pool is exhausted.
    pub fn allocate(&mut self) -> Option<NonNull<u8>> {
        // Check if we have any free blocks
        let free_index = self.free_list_head?;

        unsafe {
            let block_ptr = self.get_block_ptr(free_index);

            // Read the next free index from the current block
            let next_index = *(block_ptr as *const usize);

            // Update the free list head
            self.free_list_head = if next_index == usize::MAX {
                None
            } else {
                Some(next_index)
            };

            self.blocks_in_use += 1;

            NonNull::new(block_ptr)
        }
    }

    /// Deallocates a block, returning it to the pool.
    ///
    /// # Safety
    /// The caller must ensure that:
    /// - The pointer was originally allocated from this pool
    /// - The pointer is not used after deallocation
    /// - The pointer is only deallocated once
    pub unsafe fn deallocate(&mut self, ptr: NonNull<u8>) {
        // Calculate which block index this pointer belongs to
        let offset = ptr.as_ptr().offset_from(self.memory.as_ptr()) as usize;
        let block_index = offset / self.block_size;

        debug_assert!(
            block_index < self.total_blocks,
            "Pointer does not belong to this pool"
        );

        // Add this block back to the front of the free list
        let old_head = self.free_list_head.unwrap_or(usize::MAX);
        *(ptr.as_ptr() as *mut usize) = old_head;

        self.free_list_head = Some(block_index);
        self.blocks_in_use -= 1;
    }

    /// Returns the number of blocks currently allocated
    pub fn blocks_in_use(&self) -> usize {
        self.blocks_in_use
    }

    /// Returns the total capacity of the pool
    pub fn capacity(&self) -> usize {
        self.total_blocks
    }

    /// Returns the number of available blocks
    pub fn available(&self) -> usize {
        self.total_blocks - self.blocks_in_use
    }

    /// Returns the size of each block
    pub fn block_size(&self) -> usize {
        self.block_size
    }

    /// Checks if the pool is full (no blocks available)
    pub fn is_full(&self) -> bool {
        self.blocks_in_use == self.total_blocks
    }

    /// Checks if the pool is empty (no blocks in use)
    pub fn is_empty(&self) -> bool {
        self.blocks_in_use == 0
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        // Free the entire memory pool
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                self.block_size * self.total_blocks,
                8,
            );
            dealloc(self.memory.as_ptr(), layout);
        }
    }
}

// MemoryPool is not thread-safe by default
// For thread-safety, wrap it in a Mutex or use atomic operations
unsafe impl Send for MemoryPool {}
