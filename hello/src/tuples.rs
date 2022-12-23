// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Louis Bertson", "Burkina Faso", 24);

    println!("{} is from {} and is {}", person.0, person.1, person.2);

}