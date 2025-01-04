use std::io::{self, stdin};

fn main() {
    println!("SHAPE CALCULATOR.\nWhat would you like to do?");

    println!("\nSelect:\n'1' to calculate the area of a trapezium.\n'2' to calculate the area of a rhombus.\n'3' to calculate the area of a parallelogram.\n'4' to calculate the area of a cube.\n'5' to calculate the volume of a cylinder.\n'6' to exit the calculator
    ||.");
    calc_any();+

}

fn calc_trapezium_area(mut a: f64, mut b: f64, mut height: f64) -> f64 {
    return 0.5 * (a + b) * height;
}

fn calc_rhombus_area(mut a: f64, mut b: f64) -> f64 {
    return 0.5 * a * b;
}

fn calc_parallelogram_area(mut base: f64, mut altitude: f64) -> f64 {
    return base * altitude;
}

fn calc_cube_volume(mut length: f64) -> f64 {
    return 6.0 * length * length;
}

fn calc_cylinder_volume(mut radius: f64, mut height: f64) -> f64 {
    return 3.14 * radius * radius * height;
}

fn user_input() -> i32 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Invalid String");
    let user_choice: i32 = choice.trim().parse().expect("Idk");
    return user_choice;
}

fn convert_input(mut a: String) -> f64 {
    io::stdin().read_line(&mut a).expect("");
    let a: f64 = a.trim().parse().expect("");
    return a;
}

fn calc_any() {
    loop {
        let mut a = String::new();
        let mut b = String::new();
        let mut c = String::new();

        let input = user_input();

        if (input == 6) {
            println!("Exiting the calculator...");
            break;
        } else if (input == 1) {
            println!("Enter a of the trapezium: ");
            let a = convert_input(a);

            println!("Enter b of the trapezium: ");
            let b = convert_input(b);

            println!("Enter the height of the trapezium: ");
            let c = convert_input(c);

            let trapezium_area: f64 = calc_trapezium_area(a, b, c);
            println!("Area of the trapezium is: {}", trapezium_area)
        } else if (input == 2) {
            println!("Enter a of the rhombus: ");
            let a = convert_input(a);

            println!("Enter b of the rhombus: ");
            let b = convert_input(b);

            let rhombus_area: f64 = calc_rhombus_area(a, b);
            println!("Area of the rhombus is: {}", rhombus_area);
        } else if (input == 3) {
            println!("Enter the base of the parallelogram: ");
            let a = convert_input(a);

            println!("Enter the altitude of the parallelogram: ");
            let b = convert_input(b);

            let parallelogram_area: f64 = calc_parallelogram_area(a, b);
            println!("Area of the parallelogram is: {}", parallelogram_area);
        } else if (input == 4) {
            println!("Enter the length of the cube: ");
            let a = convert_input(a);

            let cube_volume: f64 = calc_cube_volume(a);
            println!("Area of the cube is: {}", cube_volume);
        } else if (input == 5) {
            println!("Enter the radius of the cylinder: ");
            let a = convert_input(a);

            println!("Enter the height of the cylinder: ");
            let b = convert_input(b);

            let cylinder_volume: f64 = calc_cylinder_volume(a, b);
            println!("Volume of the cylinder is: {}", cylinder_volume);
        } else {
            println!("Please select a valid choice")
        }
    }
}
