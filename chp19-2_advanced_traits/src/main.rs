use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters { // Sets the value for the Rhs type parameter instead of using default Self
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Fully Qualified Syntax - for methods
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Fully Qualified Syntax - for associated functions
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertrait Example
trait OutlinePrint: fmt::Display { // OutlinePrint trait requires implementation of fmt::Display trait
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Declared above

// struct Point { 
//     x: i32,
//     y: i32,
// }

impl OutlinePrint for Point {}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Newtype Pattern to Implement External Traits on External Types
// Want to implement Display on Vec<T>, Display trait and Vec<T> type
// are defined outside of our crate
struct Wrapper(Vec<String>); // Wrapper struct holds an instance of Vec<T>

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3}
    );

    assert_eq!(
        Millimeters(5250),
        Millimeters(250) + Meters(5)
    );

    let person = Human;
    println!("");
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("");
    println!("A baby dog is called a... {}?", Dog::baby_name());
    println!("A baby dog is called a {}!", <Dog as Animal>::baby_name()); // Call the baby_name method from the Animal trait as implemented on Dog
                                                                          // i.e. treat Dog type as Animal for this function call
    println!("");
    let point = Point{ x: 3, y: 3 };
    point.outline_print();
    println!("");

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

