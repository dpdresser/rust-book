use std::fmt;

// Type alias to save repition in code
// Note Thunk is a word for code to be evaluated at a later time
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(_f: Thunk) {}

fn returns_long_type() -> Thunk { Box::new(|| println!("hi")) }

// Use for Result<T, E>
type Result<T> = std::result::Result<T, std::io::Error>; // Code is easier to write and gives consistent interface

// Because just alias, still can use any methods that work on Result<T, E> as well as special syntax like ? operator

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;
    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn main() {
    let f: Thunk = Box::new(|| println!("hi"));
    takes_long_type(f);
    let _ = returns_long_type();
}
