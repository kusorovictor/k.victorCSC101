use std::io;

fn main() {
    println!("Welcome to Ernst & Young (EY) Global Limited.\nWe are here to get the most experienced dev from the first 10 developers that sign up to our platform.");
    get_most_experienced_dev();
}

fn dev_name(mut f_name: String) -> String {
    println!("Enter your full name: ");
    io::stdin().read_line(&mut f_name).expect("Not A String");
    return f_name.trim().to_string();
}

fn works_for(mut workplace: String) -> String {
    println!("Where do you work? ");
    io::stdin().read_line(&mut workplace).expect("Not A String");
    workplace = workplace.trim().to_string();

    if workplace.is_empty() {
        workplace = "Freelancer".to_string();
    }
    return workplace;
}

fn programming_exp(mut work_months: String) -> i32 {
    println!("How many months of programming experience do you have? ");
    io::stdin()
        .read_line(&mut work_months)
        .expect("Not A String");
    let work_months: i32 = work_months.trim().parse().expect("Not an Integer");
    return work_months;
}

fn dev_info() -> (String, String, i32) {
    let info = (
        dev_name(String::new()),
        works_for(String::new()),
        programming_exp(String::new()),
    );

    println!("Full name: {}", info.0);
    println!("Workplace: {}", info.1);
    println!("Programming Experience: {}", info.2);

    return info;
}

fn get_most_experienced_dev() {
    let mut vec_tuples: Vec<(String, String, i32)> = Vec::new();
    let mut i: i32 = 0;

    while i < 3 {
        println!("\nYou are developer {}", i + 1);
        vec_tuples.push(dev_info());
        i += 1;
    }

    let mut highest: i32 = 0;

    for (_, _, month_exp) in &vec_tuples {
        if *month_exp > highest {
            highest = *month_exp;
        }
    }

    //This is to get the tuple from the vec_tuples with the highest month_exp
    let highest_vec: Vec<&(String, String, i32)> = vec_tuples
        .iter()
        .filter(|(_, _, exp)| *exp == highest)
        .collect();

    for (name, workplace, month_exp) in highest_vec {
        println!("\nMost Experienced Programmer: ");
        println!("Name: {}", name);
        println!("Workplace: {}", workplace);
        println!("Months Programming: {}", month_exp);
    }
}
