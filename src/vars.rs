// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language
pub fn run() {
    let name = "Ross";
    let mut age = 25;

    println!("My name is {} and I am {}", name, age);
    age = 26;


    println!("My name is {} and I am {}", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables at once
    let (my_name, my_age) = ("Ross", 25);
    println!("{} is {}", my_name,my_age)

}
