// Variables hold priitive data or references to data
// Variables are immutables by default
// Rust is a block scoped language;

pub fn run() {
    let name = "Louis Bertson";
    let age = 37;

    println!("My name is {} and i am {}", name, age);

    // Define contants
    const ID:i32 = 001;

    println!("ID: {}", ID);

    // Assign multiple variables
    let (myname, myage) = ("Louis Bertson", 24);


    // Then we print the result
    println!("{} is {}", myname, myage);
}