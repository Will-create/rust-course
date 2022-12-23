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

    // get string capacity
    let capacity = hello.capacity();


    // check if empty
    let is_empty = hello.is_empty();

    // contains
    let contains = hello.contains("world");

    // replace
    let replaced = hello.replace("World", "There");


    // Split by whitespace
    for word in hello.split_whitespace() {
        println!("The word is: {}", word);
    }

    // create a string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');
    s.push('c');


    // Assertion testing
    assert_eq!(2, s.len());


    println!("{} capacity: {}, is empty? {}, contains world ? {}, so {}. New string: {}, with some assertion", hello, capacity, is_empty, contains, replaced, s);

}