use std::io;

fn main() {

    println!("1. Convert the Temperature from Celcius to Fahrenheit");
    println!("2. Convert the Temperature from Fahrenheit to Celcius");
    println!("Enter your choice (1/2) : ");

    let mut option = String::new();

    io::stdin()
       .read_line(&mut option)
       .expect("Failed to read input");
    
    let option : i32 = option.trim().parse().expect("Not a valid number");
    if option == 1 {
        ctof()
    } else if option == 2 {
        ftoc()
    } else {
        println!("Invalid Option, try again!");
    }
}

fn ctof() {
    println!("Enter the Temperature in Celcius: ");
    let mut ce = String::new();

    io::stdin()
        .read_line(&mut ce)
        .expect("Failed to read input");

    let ce : f32 = ce.trim().parse().expect("Not a valid number");
    
    let fa = (f32::from(ce) * 9.0/5.0) + 32.0;

    println!("Fahrenheit: {fa}°F");
}

fn ftoc() {
    println!("Enter the Temperature in Fahrenheit: ");
    let mut fa = String::new();

    io::stdin()
        .read_line(&mut fa)
        .expect("Failed to read input");

    let fa : f32 = fa.trim().parse().expect("Not a valid number");
    
    let c = (f32::from(fa) - 32.0) * (5.0 / 9.0);

    println!("Celcius: {c}°C");
}