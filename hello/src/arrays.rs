// Fixed list where data are the same data types

pub fn  run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Re-assign value
    numbers[0] = 9;

    // Get single value
    println!("Single value: {}", numbers[0]);


    //get array length
    println!("The array length is {}", numbers.len());

    // Arrray are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[2..5];
    println!("Slice : {:?}", slice);
}