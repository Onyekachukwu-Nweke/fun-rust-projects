# Custom Vec Implementation in Rust

A learning project that implements a simplified version of Rust's `Vec<T>` from scratch, exploring manual memory management and unsafe code.

## What's This About?

This project is my attempt at understanding how dynamic arrays work at a low level. Instead of just using `Vec<T>` from the standard library, I've built my own version using raw pointers and manual memory allocation. It's not meant to replace the standard library (please don't use this in production!), but it's been incredibly educational.

## Why Build This?

Working with Rust's safe abstractions is great, but sometimes you need to understand what's happening under the hood. This project helped me grasp:

- How dynamic arrays grow and manage memory
- When and why we use `unsafe` code
- The relationship between capacity and length
- How Drop handles cleanup properly
- Why separating allocation logic from data structure logic makes sense

## Architecture

The implementation is split into two main components:

### RawVec<T> - The Memory Manager

`RawVec` handles all the gnarly memory allocation stuff. Think of it as the foundation:

```
RawVec {
    ptr: NonNull<T>,  // Points to our heap-allocated memory
    cap: usize,       // How many T's we can fit
}
```

**What it does:**
- Allocates memory on the heap
- Grows capacity when needed (doubles each time)
- Deallocates memory when dropped
- Keeps track of how much space we have

**Pseudocode for growing:**
```
function grow():
    if capacity == 0:
        new_capacity = 1
    else:
        new_capacity = capacity * 2

    if capacity == 0:
        allocate new_capacity elements
    else:
        reallocate existing memory to new_capacity elements

    update capacity
```

### Vec<T> - The Smart Container

`Vec` uses `RawVec` internally and adds the logic for actually managing elements:

```
Vec {
    buf: RawVec<T>,  // Handles our memory
    len: usize,      // How many elements we're actually using
}
```

**Key insight:** Capacity vs Length
- **Capacity** is how much room we have (like seats in a car)
- **Length** is how many elements we're storing (like passengers currently in the car)

We can have `cap = 8` but `len = 3`, meaning we have space for 5 more elements before needing to grow.

## How Operations Work

### Push

Adding an element to the end:

```
function push(element):
    if len == capacity:
        grow()  // Double our capacity

    write element to memory[len]
    len += 1
```

### Pop

Removing from the end:

```
function pop():
    if len == 0:
        return None

    len -= 1
    element = read from memory[len]
    return Some(element)
```

Note: We don't actually erase the memory - we just decrement the length and read the value out. The next push will overwrite it.

### Insert

Inserting at any position is trickier:

```
function insert(index, element):
    assert index <= len

    if len == capacity:
        grow()

    // Shift everything from index onwards one position right
    copy memory[index..len] to memory[index+1..len+1]

    write element to memory[index]
    len += 1
```

### Remove

Similar to insert, but in reverse:

```
function remove(index):
    assert index < len

    element = read from memory[index]

    // Shift everything after index one position left
    copy memory[index+1..len] to memory[index..len-1]

    len -= 1
    return element
```

## Memory Safety

### The Drop Dance

When our `Vec` goes out of scope, we need to clean up properly:

1. **Vec's Drop:** Pops all elements one by one (this runs their destructors)
2. **RawVec's Drop:** Deallocates the memory

This separation is crucial - we drop the *elements* first, then free the *memory*. Doing it the other way around would be a use-after-free bug.

### Why All The Unsafe?

We use `unsafe` for operations the compiler can't verify:
- Writing to uninitialized memory
- Reading from raw pointers
- Calling allocation functions
- Creating slices from raw pointers

But here's the thing: the `unsafe` blocks are small and contained. The rest of our API is safe because we've carefully maintained our invariants.

## Project Structure

```
src/vec/
├── mod.rs        # Module declarations
├── core.rs       # Vec struct and basic methods
├── rawvec.rs     # RawVec memory management
├── ops.rs        # Operations (push, pop, insert, remove)
└── traits.rs     # Trait implementations (Drop, Deref, DerefMut)
```

## Running The Code

```bash
cargo run
```

This runs a test suite that exercises all the major operations. Watch the capacity double as we push more elements!

## Learning Resources

These resources were invaluable while building this:

1. **[The Rustonomicon - Implementing Vec](https://doc.rust-lang.org/nomicon/vec/vec.html)**
   The official guide to implementing Vec. This is basically the playbook.

2. **[The Rustonomicon - Working with Uninitialized Memory](https://doc.rust-lang.org/nomicon/uninitialized.html)**
   Essential reading for understanding why we use `ptr::write` instead of assignment.

3. **[std::alloc Documentation](https://doc.rust-lang.org/std/alloc/)**
   Everything about Rust's allocator interface.

4. **[Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)**
   From The Rust Book - you need to really grok ownership before diving into unsafe code.

5. **[Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)**
   Also from The Rust Book - explains the superpowers and responsibilities of unsafe.

6. **[std::ptr Documentation](https://doc.rust-lang.org/std/ptr/)**
   Details on `ptr::write`, `ptr::read`, `ptr::copy`, and friends.

## Things I Learned

- **Zero-Sized Types are weird:** We explicitly don't support them because their memory layout breaks assumptions.
- **Growing strategy matters:** Doubling the capacity gives amortized O(1) push operations.
- **`ptr::copy` vs `ptr::copy_nonoverlapping`:** We use `copy` because our source and destination ranges can overlap during insert/remove.
- **Layout is important:** We need to calculate the correct memory layout for our type before allocating.
- **Deref coercion is powerful:** By implementing `Deref<Target=[T]>`, our Vec can use all slice methods for free.

## Limitations

This is a learning implementation, so it's missing a lot of std::Vec features:

- No support for zero-sized types
- No `with_capacity` constructor
- No `reserve`, `shrink_to_fit`, or other capacity management
- No `extend`, `append`, or bulk operations
- No custom allocators
- Probably not as optimized as the real thing

## What's Next?

Some ideas for extending this:
- Add iterators (IntoIter, Iter, IterMut)
- Implement more Vec methods (extend, append, etc.)
- Add drain() functionality
- Implement FromIterator and other traits
- Maybe try implementing VecDeque using RawVec?

## Final Thoughts

Building this was humbling. The standard library makes it look easy, but there's a ton of careful thought behind every operation. If you're learning Rust, I highly recommend working through The Rustonomicon's Vec implementation - it really solidifies your understanding of ownership, memory management, and unsafe code.

---

**Note:** This code is for educational purposes. Don't use it in production. The standard library's Vec is battle-tested, optimized, and handles edge cases this implementation doesn't.
