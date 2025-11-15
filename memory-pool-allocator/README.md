# Memory Pool Allocator in Rust

A learning project that implements a fixed-size block allocator with O(1) allocation and deallocation, exploring memory management strategies used in performance-critical systems.

## What's This About?

This project implements a memory pool allocator (also known as a fixed-size block allocator) from scratch. Instead of asking the system allocator for memory every time we need it, we pre-allocate a large chunk upfront and manage it ourselves. This is a common pattern in game engines, embedded systems, and high-performance applications where allocation speed matters more than flexibility.

## Why Build This?

Memory pools solve a specific problem: the cost of frequent allocations. When you know you'll be creating and destroying lots of objects of the same size (think bullets in a game, network packets, or temporary objects), a memory pool can be orders of magnitude faster than the system allocator. Building this helped me understand:

- How to manage raw memory without relying on the standard allocator
- The tradeoff between flexibility and performance
- How intrusive data structures work (using free blocks to store the free list)
- Why fixed-size allocation is so much faster than general-purpose allocation
- The importance of proper cleanup in unsafe code

## The Core Idea

Instead of this:
```
Every allocation → System allocator → Maybe malloc → Kernel → Find free memory → Update bookkeeping
Every deallocation → System allocator → Maybe free → Complex bookkeeping
```

We do this:
```
Setup: Allocate ONE big chunk
Every allocation → Pop from free list (1 pointer operation)
Every deallocation → Push to free list (1 pointer operation)
```

## Architecture

The entire allocator is built around a single structure:

```
MemoryPool {
    memory: NonNull<u8>,      // Pointer to our big memory chunk
    block_size: usize,        // Size of each block
    total_blocks: usize,      // How many blocks we carved up
    free_list_head: Option<usize>,  // Index of first free block
    blocks_in_use: usize,     // How many are currently allocated
}
```

### The Clever Part: Intrusive Free List

Here's the key insight that makes this work:

**When a block is free, we don't need to store data in it - we can use it to store bookkeeping information!**

Each free block stores the index of the *next* free block. It's a linked list, but instead of separate nodes, the list nodes ARE the free blocks themselves.

```
Memory Layout:
┌─────────┬─────────┬─────────┬─────────┬─────────┐
│ Block 0 │ Block 1 │ Block 2 │ Block 3 │ Block 4 │
└─────────┴─────────┴─────────┴─────────┴─────────┘

Initial State (all free):
Block 0: [next = 1]
Block 1: [next = 2]
Block 2: [next = 3]
Block 3: [next = 4]
Block 4: [next = MAX]  // End of list
free_list_head = Some(0)

After allocating blocks 0 and 2:
Block 0: [user data] ✓ allocated
Block 1: [next = 3]
Block 2: [user data] ✓ allocated
Block 3: [next = 4]
Block 4: [next = MAX]
free_list_head = Some(1)
```

This is called an **intrusive data structure** - it uses the memory it's managing to store its own metadata.

## How Operations Work

### Initialization

When we create a pool, we set up the free list:

```
function new(block_size, total_blocks):
    // Ensure blocks are large enough to hold a pointer
    actual_block_size = max(block_size, sizeof(usize))

    // Allocate the entire pool in one go
    total_size = actual_block_size * total_blocks
    memory = allocate(total_size)

    // Build the free list: make each block point to the next
    for i in 0..total_blocks:
        block_ptr = memory + (i * actual_block_size)
        if i < total_blocks - 1:
            write next_index (i + 1) to block_ptr
        else:
            write MAX to block_ptr  // End of list

    free_list_head = Some(0)
    blocks_in_use = 0
```

**Why this matters:** We pay the allocation cost once, upfront. Every subsequent allocation is just pointer manipulation.

### Allocation

Getting a block is beautifully simple:

```
function allocate():
    if free_list_head is None:
        return None  // Pool exhausted

    // Get the index of the first free block
    free_index = unwrap(free_list_head)

    // Calculate its memory address
    block_ptr = memory + (free_index * block_size)

    // Read what the next free block is (stored IN this block)
    next_free_index = read usize from block_ptr

    // Update the head of the free list
    if next_free_index == MAX:
        free_list_head = None
    else:
        free_list_head = Some(next_free_index)

    blocks_in_use += 1
    return Some(block_ptr)
```

**Time complexity:** O(1) - just a few pointer operations.

**Key insight:** We're reading metadata from the block itself, then giving that block to the user. The user will overwrite the metadata, but that's fine - they own the block now.

### Deallocation

Returning a block is equally simple:

```
function deallocate(ptr):
    // Figure out which block this is
    offset = ptr - memory
    block_index = offset / block_size

    // Add it to the front of the free list
    old_head_index = free_list_head.unwrap_or(MAX)

    // Write the old head index into this block
    write old_head_index to ptr

    // Make this block the new head
    free_list_head = Some(block_index)

    blocks_in_use -= 1
```

**Time complexity:** O(1) - just a few writes and arithmetic.

**Key insight:** We're giving the block back by putting it at the front of the free list. The data the user stored is overwritten with our free list pointer, but the user promised not to use the block anymore.

## The Tricky Parts

### Challenge 1: Minimum Block Size

**Problem:** What if the user wants 1-byte blocks, but we need at least `sizeof(usize)` bytes to store the free list pointers?

**Solution:** We enforce a minimum block size:

```rust
let block_size = block_size.max(std::mem::size_of::<usize>());
```

This ensures every block can hold a pointer to the next free block.

### Challenge 2: Safety of Deallocation

**Problem:** How do we know the pointer the user is deallocating actually came from our pool?

**Solution:** We calculate the block index and use `debug_assert!` to catch mistakes in debug builds:

```rust
let offset = ptr.as_ptr().offset_from(self.memory.as_ptr()) as usize;
let block_index = offset / self.block_size;
debug_assert!(
    block_index < self.total_blocks,
    "Pointer does not belong to this pool"
);
```

In release builds, passing a wrong pointer is undefined behavior - but this is documented as a safety requirement.

### Challenge 3: Using usize::MAX as "None"

**Problem:** We need to mark the end of the free list, but we're storing indices as `usize` in the blocks themselves.

**Solution:** We use `usize::MAX` as a sentinel value meaning "no next block":

```rust
let next_index = if i + 1 < self.total_blocks {
    i + 1
} else {
    usize::MAX  // End marker
};
```

This is safe because we'll never have `usize::MAX` blocks in a pool (that would require more memory than addressable).

### Challenge 4: Drop Order

**Problem:** What if there are still allocated blocks when the pool is dropped?

**Solution:** We don't handle them specially - it's the user's responsibility to deallocate all blocks before dropping the pool. This is documented behavior.

```rust
impl Drop for MemoryPool {
    fn drop(&mut self) {
        // Just free the entire memory chunk
        unsafe {
            let layout = Layout::from_size_align_unchecked(
                self.block_size * self.total_blocks,
                8,
            );
            dealloc(self.memory.as_ptr(), layout);
        }
    }
}
```

**Why not track individual blocks?** Because:
1. It would add overhead to every allocation/deallocation
2. Users of memory pools are expected to manage their allocations carefully
3. This is a low-level primitive - safety is the caller's responsibility

## Performance Characteristics

| Operation | Time Complexity | Allocation? |
|-----------|----------------|-------------|
| `new()` | O(n) where n = total_blocks | Yes - one big allocation |
| `allocate()` | O(1) | No |
| `deallocate()` | O(1) | No |
| `drop()` | O(1) | No - one deallocation |

**Compare to system allocator:**
- System `malloc`: O(log n) to O(n) depending on implementation
- System `free`: Similar complexity

**Memory overhead:**
- System allocator: Usually 8-16 bytes per allocation
- Memory pool: Zero per allocation (uses the blocks themselves!)

## Project Structure

```
src/
├── lib.rs       # Module exports and test module
├── pool.rs      # MemoryPool implementation
├── tests.rs     # Comprehensive test suite
└── main.rs      # Example usage and demos
```

## Running The Code

Run the examples:
```bash
cargo run
```

Run the test suite:
```bash
cargo test
```

The tests cover:
- Basic allocation and deallocation
- Pool exhaustion (what happens when full)
- Reallocation after freeing blocks
- Data isolation between blocks
- Storing custom structs
- Edge cases and panics

## Use Cases

Memory pools shine in these scenarios:

1. **Game Development**
   ```rust
   // Particle system with 1000 particles
   let mut particle_pool = MemoryPool::new(size_of::<Particle>(), 1000);
   // Spawn/despawn particles with O(1) operations
   ```

2. **Network Servers**
   ```rust
   // Pool for incoming packet buffers
   let mut packet_pool = MemoryPool::new(MTU_SIZE, 10000);
   // Grab buffers as packets arrive, return when done
   ```

3. **Object Pooling**
   ```rust
   // Reusable database connection objects
   let mut conn_pool = MemoryPool::new(size_of::<DbConnection>(), 20);
   ```

4. **Embedded Systems**
   - Deterministic allocation time (no malloc unpredictability)
   - No fragmentation within the pool
   - Bounded memory usage

## Things I Learned

- **Intrusive data structures are brilliant:** Using free blocks to store the free list eliminates all per-block overhead.

- **Fixed-size allocation is fundamentally simpler:** We don't need complex algorithms to find a block that fits - they all fit!

- **NonNull is your friend:** It's like `*mut T` but with null-pointer optimization and better semantics.

- **offset_from is powerful:** Converting pointers to indices lets us do arithmetic safely.

- **Alignment matters:** We use alignment of 8 for the entire pool, which works for most types, but a production implementation would calculate alignment from the block size.

- **Debug assertions are free in release:** Using `debug_assert!` for sanity checks gives us safety in development without runtime cost in production.

## Comparison to Other Allocators

### vs. System Allocator (malloc/free)
- **Pros:** Much faster, no fragmentation, predictable performance
- **Cons:** Fixed block size, must pre-allocate, wastes space if blocks aren't full

### vs. Bump Allocator
- **Pros:** Can deallocate individual blocks
- **Cons:** Slower allocation (need to follow free list), more complex

### vs. Slab Allocator
- **Similar:** Both are fixed-size block allocators
- **Difference:** Slab allocators usually have multiple pools of different sizes

## Limitations

This is a learning implementation. Production memory pools often add:

- **Thread safety:** Atomic operations or per-thread pools
- **Alignment control:** Proper alignment for any type
- **Debugging features:** Guard pages, use-after-free detection, leak detection
- **Statistics:** Track allocation patterns, fragmentation
- **Multi-size support:** Multiple pools for different sizes
- **Growing:** Ability to allocate more blocks if needed

## What's Next?

Ideas for extending this:

- Implement a thread-safe version using atomics
- Add a `PoolBox<T>` type that auto-deallocates on drop
- Create a `PoolVec` that uses pool allocation for its elements
- Implement a buddy allocator for variable-size allocation
- Add instrumentation to track allocation patterns

## Learning Resources

These helped me understand memory pool allocators:

1. **[The Memory Management Reference - Pool Allocation](http://www.memorymanagement.org/)**
   Great overview of different allocation strategies.

2. **[Game Programming Patterns - Object Pool](https://gameprogrammingpatterns.com/object-pool.html)**
   Explains the pattern from a game dev perspective.

3. **[std::alloc Documentation](https://doc.rust-lang.org/std/alloc/)**
   How to interact with Rust's allocator.

4. **[The Rustonomicon - Working with Uninitialized Memory](https://doc.rust-lang.org/nomicon/uninitialized.html)**
   Essential for understanding why we use `ptr::write`.

5. **[What Every Programmer Should Know About Memory](https://people.freebsd.org/~lstewart/articles/cpumemory.pdf)**
   Deep dive into memory hierarchies and cache effects.

## Real-World Examples

Many systems use pool allocation:

- **Linux Kernel:** Slab allocator for kernel objects
- **jemalloc:** Uses size-class pools internally
- **Unity Engine:** Object pooling system
- **Nginx:** Connection and buffer pools
- **tcmalloc:** Google's thread-caching allocator

## Final Thoughts

Memory pools are a great example of trading generality for performance. By accepting the constraint of fixed-size blocks, we gain extremely fast, predictable allocation with zero fragmentation. They're not always the right tool, but when they fit your use case, they're hard to beat.

Building this taught me that sometimes the best data structures use the memory they're managing. The intrusive free list is elegant - there's no separate bookkeeping, no wasted space, just blocks pointing to other blocks until they're needed.

---

**Note:** This code is for educational purposes. For production use, consider battle-tested crates like `typed-arena`, `slotmap`, or `generational-arena` that provide similar functionality with more features and safety guarantees.
