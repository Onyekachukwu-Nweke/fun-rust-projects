use memory_pool_allocator::MemoryPool;

fn main() {
    println!("=== Memory Pool Allocator Demo ===\n");

    // Example 1: Basic usage
    basic_usage_example();

    println!();

    // Example 2: Storing custom structs
    struct_storage_example();

    println!();

    // Example 3: Pool exhaustion and reuse
    pool_exhaustion_example();
}

fn basic_usage_example() {
    println!("Example 1: Basic Allocation");
    println!("---------------------------");

    // Create a pool with 10 blocks of 64 bytes each
    let mut pool = MemoryPool::new(64, 10);

    println!("Created pool: {} blocks of {} bytes", pool.capacity(), pool.block_size());
    println!("Available: {}", pool.available());

    // Allocate some blocks
    let block1 = pool.allocate().unwrap();
    let block2 = pool.allocate().unwrap();
    let block3 = pool.allocate().unwrap();

    println!("Allocated 3 blocks");
    println!("In use: {}, Available: {}", pool.blocks_in_use(), pool.available());

    // Write data to a block
    unsafe {
        let ptr = block1.as_ptr() as *mut u64;
        *ptr = 42;
        println!("Wrote value {} to block 1", *ptr);
    }

    // Deallocate blocks
    unsafe {
        pool.deallocate(block1);
        pool.deallocate(block2);
        pool.deallocate(block3);
    }

    println!("Deallocated all blocks");
    println!("In use: {}, Available: {}", pool.blocks_in_use(), pool.available());
}

fn struct_storage_example() {
    println!("Example 2: Storing Custom Structs");
    println!("----------------------------------");

    #[derive(Debug)]
    struct Player {
        id: u32,
        x: f32,
        y: f32,
        health: i32,
    }

    // Create a pool sized for Player structs
    let mut pool = MemoryPool::new(std::mem::size_of::<Player>(), 100);

    println!("Created pool for {} Player objects", pool.capacity());

    // Allocate and initialize players
    let mut player_blocks = Vec::new();

    for i in 0..5 {
        if let Some(block) = pool.allocate() {
            unsafe {
                let player_ptr = block.as_ptr() as *mut Player;
                *player_ptr = Player {
                    id: i,
                    x: i as f32 * 10.0,
                    y: i as f32 * 5.0,
                    health: 100,
                };
                println!("Created player: {:?}", &*player_ptr);
                player_blocks.push(block);
            }
        }
    }

    println!("Pool status - In use: {}/{}", pool.blocks_in_use(), pool.capacity());

    // Clean up
    unsafe {
        for block in player_blocks {
            pool.deallocate(block);
        }
    }

    println!("All players deallocated");
}

fn pool_exhaustion_example() {
    println!("Example 3: Pool Exhaustion and Reuse");
    println!("-------------------------------------");

    // Small pool to demonstrate exhaustion
    let mut pool = MemoryPool::new(32, 3);

    println!("Created small pool with {} blocks", pool.capacity());

    // Allocate all blocks
    let b1 = pool.allocate().unwrap();
    let b2 = pool.allocate().unwrap();
    let b3 = pool.allocate().unwrap();

    println!("Allocated all {} blocks", pool.blocks_in_use());
    println!("Is pool full? {}", pool.is_full());

    // Try to allocate when exhausted
    match pool.allocate() {
        Some(_) => println!("Unexpectedly got a block!"),
        None => println!("Pool is exhausted - allocation returned None"),
    }

    // Free one block
    unsafe {
        pool.deallocate(b2);
    }

    println!("Deallocated 1 block, available: {}", pool.available());

    // Now we can allocate again
    match pool.allocate() {
        Some(_) => println!("Successfully reallocated from freed block"),
        None => println!("Failed to allocate"),
    }

    // Clean up remaining blocks
    unsafe {
        pool.deallocate(b1);
        pool.deallocate(b3);
    }

    println!("Pool is empty? {}", pool.is_empty());
}
