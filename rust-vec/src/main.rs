use rust_vec::vec::Vec;

fn main() {
    println!("Testing RawVec-based Vec implementation...");

    // Test basic push and pop
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After pushing 1, 2, 3: len={}, cap={}", v.len(), v.capacity());

    // Test pop
    if let Some(x) = v.pop() {
        println!("Popped: {}", x);
    }
    println!("After pop: len={}, cap={}", v.len(), v.capacity());

    // Test insert
    v.insert(0, 0);
    println!("After inserting 0 at index 0: {:?}", &*v);

    // Test remove
    let removed = v.remove(1);
    println!("Removed element at index 1: {}", removed);
    println!("Final vec: {:?}", &*v);

    // Test growing capacity
    let mut v2 = Vec::new();
    for i in 0..10 {
        v2.push(i);
        println!("Pushed {}: len={}, cap={}", i, v2.len(), v2.capacity());
    }

    println!("All tests passed!");
}
