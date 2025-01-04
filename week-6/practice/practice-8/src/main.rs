use std::io;
use std::io::Write;

fn main() {
    let mut student_name: Vec<String> = Vec::new();
    let mut matriculation_no: Vec<String> = Vec::new();
    let mut department: Vec<String> = Vec::new();
    let mut level: Vec<u32> = Vec::new();
    let mut count: Vec<u32> = Vec::new();

    println!("\n\n\nWelcome! This is PAU's Student Management Information System");
    println!("Kindly follow the instructions below:");

    loop {
        //Basically prompting the user to enter their info
        let mut name = String::new();
        println!("Enter student's fullname");
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read input");
        student_name.push(name);

        let mut matric_no = String::new();
        println!("Enter student's matriculation number");
        io::stdin()
            .read_line(&mut matric_no)
            .expect("Failed to read input");
        matriculation_no.push(matric_no);

        let mut dep = String::new();
        println!("Enter student's department");
        io::stdin()
            .read_line(&mut dep)
            .expect("Failed to read input");
        department.push(dep);

        let mut lvl = String::new();
        println!("Enter student's level");
        io::stdin()
            .read_line(&mut lvl)
            .expect("Failed to read input");
        let lvl: u32 = lvl.trim().parse().expect("Invalid input");

        //Make sure the user inputs a reasonable level
        if lvl % 100 == 0 && (lvl >= 100 && lvl <= 600) {
            level.push(lvl);
        }

        let mut index = 0;
        index += 1;
        count.push(index);

        println!("Would you like to input another student's information?  (y/ n)");
        let mut enter = String::new();
        io::stdin().read_line(&mut enter).expect("INVALID INPUT");
        let response = enter.trim().to_lowercase().to_string();

        if response == "y" || response == "yes" {
        } else {
            break;
        }
    }
    //Create the text file
    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Failed to create file");

    //Basically just looping thru the vectors and assigning each value to a variable
    for i in 0..count.len() {
        let name = if i < count.len() {
            "Full Name: ".to_string() + &student_name[i]
        } else {
            "null".to_string()
        };
        let matric_no = if i < count.len() {
            "Matric. Number: ".to_string() + &matriculation_no[i]
        } else {
            "null".to_string()
        };
        let department = if i < count.len() {
            "Department: ".to_string() + &department[i]
        } else {
            "null".to_string()
        };
        let level = if i < count.len() {
            "Level: ".to_string() + &level[i].to_string()
        } else {
            "null".to_string()
        };

        //writing the assigned values to the file
        file.write_all(name.as_bytes()).expect("Failed to write");

        file.write_all(matric_no.as_bytes())
            .expect("Failed to write ");

        file.write_all(department.as_bytes())
            .expect("Failed to write ");

        file.write_all(&level.as_bytes()).expect("Failed to write ");

        file.write("\n".to_string().as_bytes())
            .expect("Failed to write");

        println!("File has been Created.");
    }
}
