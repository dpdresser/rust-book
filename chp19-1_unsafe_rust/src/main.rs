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
}
