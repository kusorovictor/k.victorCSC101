use std::io;

fn main() {
    let mut choice = String::new();
    let mut age = String::new();

    println!("How old are you?");
    io::stdin().read_line(&mut age).expect("Invalid String");
    let age: i32 = age.trim().parse().expect("Invalid Integer.");

    println!("Select how experienced you are.\n1.Experienced. \n2.Inexperienced. ");
    io::stdin().read_line(&mut choice).expect("Invalid String");
    let choice: i32 = choice.trim().parse().expect("Invalid Integer.");

    if choice == 1 {
        println!("Your salary incentive is N100K");
    } else if choice == 2 && age >= 40 {
        println!("Your salary incentive is N1,560,000");
    } else if choice == 2 && (age >= 30 && age < 40) {
        println!("Your salary incentive is N1,480,000");
    } else if choice == 2 && age < 30 {
        println!("Your salary incentive is N1,300,000");
    } else {
        println!("You have no salary incentive! Re-check your age and experience level!");
    }
}
