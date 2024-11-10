use std::io;

fn main() {
    println!("\nStudent Information Management System");

    let error_msg = "Failed to read input";

    //input name
    println!("\nPlease enter your name: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect(&error_msg);
    println!("Your name is: {}", name);

    //input age
    println!("\nEnter your age: ");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect(&error_msg);

    let age: i32 = age.trim().parse().expect("Input not an integer");
    println!("Your age is: {}", age);
}
