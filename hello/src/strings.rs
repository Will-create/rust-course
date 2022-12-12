// There are two types of strings

// Primitive str = Immutable fixed-length string somewhere in memory
// String = growable, heap-allocated data structure - Use when you need to modify or own string data


pub fn run() {
    let mut hello = String::from("Hello");
    // Length
    let len = hello.len();

    println!("{}", len);

    // Push character
    hello.push('w');

    // Push a string'

    hello.push_str(" World");
    println!("{}", hello);

}