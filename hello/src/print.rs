pub fn run() {
    // print to console
    println!("Hello from print.rs file");

    // Basical fromating
    println!("{}  is from {}!", "Louis", "Burkina Faso");

    // Positional arguments
    println!("{0} is from {1} and {0} likes to {2}", "Louis", "Burkina Faso", "Code");

    // Named arguments
    println!("{name} is from {country} and {name} likes {activity}", name = "Louis", country = "Burkina Faso", activity = "Coding");

    // Placeholders traits
    println!("Binary : {:b} Hex : {:x} and Octal : {:o}", 10, 10, 10);

    // Placeholders for debug traits
    println!("{:?}", (12, true, "Hello"));

    // Basic macth
    println!("10 + 10 = {}", 10 + 10);
}