use super::*;

#[test]
fn test_pool_creation() {
    let pool = MemoryPool::new(64, 10);
    assert_eq!(pool.capacity(), 10);
    assert_eq!(pool.blocks_in_use(), 0);
    assert_eq!(pool.available(), 10);
    assert!(pool.is_empty());
    assert!(!pool.is_full());
}

#[test]
fn test_single_allocation() {
    let mut pool = MemoryPool::new(64, 10);

    let block = pool.allocate();
    assert!(block.is_some());
    assert_eq!(pool.blocks_in_use(), 1);
    assert_eq!(pool.available(), 9);
}

#[test]
fn test_multiple_allocations() {
    let mut pool = MemoryPool::new(64, 5);

    let mut blocks = Vec::new();
    for i in 0..5 {
        let block = pool.allocate();
        assert!(block.is_some(), "Allocation {} failed", i);
        blocks.push(block.unwrap());
    }

    assert_eq!(pool.blocks_in_use(), 5);
    assert!(pool.is_full());
    assert_eq!(pool.available(), 0);
}

#[test]
fn test_exhaustion() {
    let mut pool = MemoryPool::new(32, 3);

    // Allocate all blocks
    let _b1 = pool.allocate().unwrap();
    let _b2 = pool.allocate().unwrap();
    let _b3 = pool.allocate().unwrap();

    // Pool should be exhausted
    assert!(pool.is_full());
    let b4 = pool.allocate();
    assert!(b4.is_none(), "Pool should be exhausted");
}

#[test]
fn test_deallocation() {
    let mut pool = MemoryPool::new(64, 5);

    let b1 = pool.allocate().unwrap();
    let b2 = pool.allocate().unwrap();

    assert_eq!(pool.blocks_in_use(), 2);

    unsafe {
        pool.deallocate(b1);
    }

    assert_eq!(pool.blocks_in_use(), 1);
    assert_eq!(pool.available(), 4);

    unsafe {
        pool.deallocate(b2);
    }

    assert_eq!(pool.blocks_in_use(), 0);
    assert!(pool.is_empty());
}

#[test]
fn test_reallocation_after_deallocation() {
    let mut pool = MemoryPool::new(128, 3);

    // Allocate all blocks
    let b1 = pool.allocate().unwrap();
    let b2 = pool.allocate().unwrap();
    let b3 = pool.allocate().unwrap();

    assert!(pool.is_full());

    // Free one block
    unsafe {
        pool.deallocate(b2);
    }

    assert!(!pool.is_full());
    assert_eq!(pool.available(), 1);

    // Should be able to allocate again
    let b4 = pool.allocate();
    assert!(b4.is_some());
    assert!(pool.is_full());

    // Clean up
    unsafe {
        pool.deallocate(b1);
        pool.deallocate(b3);
        pool.deallocate(b4.unwrap());
    }
}

#[test]
fn test_write_and_read_data() {
    let mut pool = MemoryPool::new(256, 2);

    let block = pool.allocate().unwrap();

    // Write data to the block
    unsafe {
        let ptr = block.as_ptr() as *mut u64;
        *ptr = 0xDEADBEEF;

        // Read it back
        let value = *ptr;
        assert_eq!(value, 0xDEADBEEF);

        pool.deallocate(block);
    }
}

#[test]
fn test_multiple_blocks_isolation() {
    let mut pool = MemoryPool::new(128, 3);

    let b1 = pool.allocate().unwrap();
    let b2 = pool.allocate().unwrap();
    let b3 = pool.allocate().unwrap();

    // Write different values to each block
    unsafe {
        *(b1.as_ptr() as *mut u32) = 111;
        *(b2.as_ptr() as *mut u32) = 222;
        *(b3.as_ptr() as *mut u32) = 333;

        // Verify each block retained its own value
        assert_eq!(*(b1.as_ptr() as *const u32), 111);
        assert_eq!(*(b2.as_ptr() as *const u32), 222);
        assert_eq!(*(b3.as_ptr() as *const u32), 333);

        pool.deallocate(b1);
        pool.deallocate(b2);
        pool.deallocate(b3);
    }
}

#[test]
fn test_allocate_deallocate_cycle() {
    let mut pool = MemoryPool::new(64, 2);

    // Do several allocation/deallocation cycles
    for _ in 0..5 {
        let b = pool.allocate().unwrap();
        assert_eq!(pool.blocks_in_use(), 1);

        unsafe {
            pool.deallocate(b);
        }

        assert_eq!(pool.blocks_in_use(), 0);
    }
}

#[test]
fn test_block_size_adjustment() {
    // Block size smaller than usize should be adjusted
    let pool = MemoryPool::new(1, 5);
    assert!(pool.block_size() >= std::mem::size_of::<usize>());
}

#[test]
#[should_panic(expected = "Block size must be greater than 0")]
fn test_zero_block_size_panics() {
    let _pool = MemoryPool::new(0, 10);
}

#[test]
#[should_panic(expected = "Total blocks must be greater than 0")]
fn test_zero_blocks_panics() {
    let _pool = MemoryPool::new(64, 0);
}

#[test]
fn test_large_pool() {
    let mut pool = MemoryPool::new(1024, 1000);

    let mut blocks = Vec::new();
    for _ in 0..500 {
        if let Some(block) = pool.allocate() {
            blocks.push(block);
        }
    }

    assert_eq!(pool.blocks_in_use(), 500);
    assert_eq!(pool.available(), 500);

    // Deallocate all
    unsafe {
        for block in blocks {
            pool.deallocate(block);
        }
    }

    assert!(pool.is_empty());
}

#[test]
fn test_struct_storage() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    let mut pool = MemoryPool::new(std::mem::size_of::<Point>(), 5);

    let block = pool.allocate().unwrap();

    unsafe {
        let point_ptr = block.as_ptr() as *mut Point;
        *point_ptr = Point { x: 3.14, y: 2.71 };

        let retrieved = &*point_ptr;
        assert_eq!(retrieved.x, 3.14);
        assert_eq!(retrieved.y, 2.71);

        pool.deallocate(block);
    }
}
