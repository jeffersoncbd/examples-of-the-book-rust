use add_one::add_one;
use add_two::add_two;

fn main() {
    let current_age = 27;
    println!("Hello, world! / New age: {}", add_two(add_one(current_age)));
}
