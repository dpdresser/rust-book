fn main() {
    let mut num = 5;

    // Can create raw pointers in safe code; but can't dereference raw pointers outside of an unsafe block
    let r1 = &num as *const i32; // immutable raw pointer
    let r2 = &mut num as *mut i32; // mutable raw pointer

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Unsafe functions
    unsafe fn dangerous() {}

    // dangerous(); -> compile error

    unsafe {
        dangerous();
    }

    // Wrapping unsafe code in safe abstraction
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..]; // reference
    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

// Example of method in standard library, this first version won't compile
// Rust's borrow checker can't understand that we're borrowing different parts of the slice
// When we know code is okay, but Rust doens't, time to reach for unsafe code

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();

//     assert!(mid <= len);

//     (&mut values[..mid], &mut values[mid..]) // cannot borrow `*values` as mutable more than once at a time
// }

use std::slice;

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // slice::from_raw_parts is unsafe because it takes a raw pointer
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), // and must trust this pointer is valid, with 
        )                                                                        // the assert above we can guarantee our pointers
    }                                                                            // will always be valid
}

// the fn split_at_mut isn't marked as unsafe, and we can call the function from safe Rust; we've created a safe abstraction to the
// unsafe code
