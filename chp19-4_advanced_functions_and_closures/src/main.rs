// Function Pointer example

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 { // Specifiy parameter f is an fn that takes one parameter of type i32 and returns an i32
    f(arg) + f(arg)
}

#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 5);
    println!("The answer is: {answer}");

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); // Name of each enum variant also becomes initializer function
    println!("{:?}", list_of_statuses);
}
