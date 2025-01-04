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

        let mut lvls = String::new();
        println!("Enter student's level");
        io::stdin()
            .read_line(&mut lvls)
            .expect("Failed to read input");
        let lvl: u32 = lvls.trim().parse().expect("Invalid input");
        if lvl % 100 == 0 {
            level.push(lvl)
        };

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
    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Failed to create file");

    file.write_all("Student Name                 Matric.No                 Department                 level\n\n\n".as_bytes())
        .expect("Failed to write header");

    for i in 0..count.len() {
        let a = if i < count.len() {
            &student_name[i]
        } else {
            &String::from("")
        };
        let b = if i < count.len() {
            &matriculation_no[i]
        } else {
            &String::from("")
        };
        let c = if i < count.len() {
            &department[i]
        } else {
            &String::from("")
        };
        let d = if i < count.len() { &level[i] } else { &100 };

        // Write each column with spaces for alignment
        file.write_all(a.as_bytes()).expect("Failed to write");
        file.write_all("            ".as_bytes())
            .expect("Failed to write spaces");

        file.write_all(b.as_bytes()).expect("Failed to write ");
        file.write_all("                ".as_bytes())
            .expect("Failed to write spaces");

        file.write_all(c.as_bytes()).expect("Failed to write ");
        file.write_all("                ".as_bytes())
            .expect("Failed to write spaces");

        file.write_all(&d.to_be_bytes()).expect("Failed to write ");
        file.write_all("\n".as_bytes())
            .expect("Failed to write newline");

        println!("File has been Created.");
    }
}
