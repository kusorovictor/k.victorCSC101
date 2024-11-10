use std::io;

fn main() {
    //prompt the user to enter a b and c
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Welcome to my Quadratic Equation Calculator!!!");
    
    println!("\nEnter the value of the co-effiecent of a: ");
    //type casting Strings to integers
    io::stdin()
        .read_line(&mut input1)
        .expect("Not a valid String");
    let a: f64 = input1.trim().parse().expect("Not a valid integer!");

    println!("Enter the value of the co-effiecent of b: ");
    //type casting Strings to integers
    io::stdin()
        .read_line(&mut input2)
        .expect("Not a valid String");
    let b: f64 = input2.trim().parse().expect("Not a valid integer!");

    println!("Enter the value of the co-effiecent of c: ");
    //type casting Strings to integers
    io::stdin()
        .read_line(&mut input3)
        .expect("Not a valid String");
    let c: f64 = input3.trim().parse().expect("Not a valid integer!");

    //Calculate the discriminant of the Quadratic function
    let discriminant = b.powf(2.0) - (4.0 * a * c);

    println!("Discriminant: {}", discriminant);

    if discriminant < 0.0 {
        println!("There are no real roots for this equation.");
    } else {
        let answer_1 = -b + (discriminant.sqrt() / 2.0 * a);
        let answer_2 = -b - (discriminant.sqrt() / (2.0 * a));
        println!("Roots: {}, {}", answer_1, answer_2);
    }
}
