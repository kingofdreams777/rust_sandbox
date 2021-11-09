pub fn run() {
    println!("Hello from the print.rs file");

    println!("{} is from {}", "Ross", "San Diego");

    println!("{0} is from {1} and {0} likes to {2}", "Ross", "San Diego", "code and surf");

    //Named arguments
    println!("{name} likes to play {activity}", name="Ross", activity = "Surf");

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    //Placeholder for debug trait
    println!("{:?}", (12,true,"hello"));

    //Basic math
    println!("10 + 10 = {}", 10 + 10);
}