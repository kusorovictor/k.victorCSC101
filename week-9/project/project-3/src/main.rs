use std::{
    fs::{File, OpenOptions},
    io::Write,
};

fn main() {
    let file = OpenOptions::new()
        .append(true)
        .open("ministers.txt")
        .unwrap();
    loop {
        let minister = input_minister_info();
        let minister_string = format!(
            "    {}           {}               {}\n",
            minister.name.trim(),
            minister.ministry.trim(),
            minister.geopolitical_zone.trim()
        );

        write_to_file(file, minister_string);

        if end() {
            break;
        }
    }
}

fn write_to_file(mut file: File, str: String) {
    file.write_all(str.as_bytes())
        .expect("Failed to write to file.");
}

struct Minister {
    name: String,
    ministry: String,
    geopolitical_zone: String,
}

fn input_minister_info() -> Minister {
    let name = input_name();
    let ministry = input_ministry();
    let geopolitical_zone = input_geopolitical_zone();

    assign_minister_info(name, ministry, geopolitical_zone)
}

fn input_name() -> String {
    let mut name = String::new();
    println!("Enter the name of the minister: ");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    name
}

fn input_ministry() -> String {
    let mut ministry = String::new();
    println!("Enter the ministry of the minister: ");
    std::io::stdin()
        .read_line(&mut ministry)
        .expect("Failed to read line");
    ministry
}

fn input_geopolitical_zone() -> String {
    let mut geopolitical_zone = String::new();
    println!("Enter the geopolitical zone of the minister: ");
    std::io::stdin()
        .read_line(&mut geopolitical_zone)
        .expect("Failed to read line");
    geopolitical_zone
}

fn end() -> bool {
    let mut end = String::new();
    println!("Do you want to add another minister? (y/n)");
    std::io::stdin()
        .read_line(&mut end)
        .expect("Failed to read line");

    end.trim().eq("n")
}

fn assign_minister_info(name: String, ministry: String, geopolitical_zone: String) -> Minister {
    let minister = Minister {
        name,
        ministry,
        geopolitical_zone,
    };

    minister
}
