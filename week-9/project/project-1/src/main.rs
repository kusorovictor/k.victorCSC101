use std::{
    fs::{File, OpenOptions},
    io::Write,
    vec,
};

fn main() {
    let larger_vector = larger_fn();
    let stout_vector = stout_fn();
    let non_alcoholic_vector = non_alcoholic_fn();

    let mut file_drinks = std::fs::File::create("drinks.txt").unwrap();

    write_vector_to_file(larger_vector, &mut file_drinks, "Larger: ".to_string());
    write_vector_to_file(stout_vector, &mut file_drinks, "Stout: ".to_string());
    write_vector_to_file(
        non_alcoholic_vector,
        &mut file_drinks,
        "Non-alcoholic:".to_string(),
    );
}

fn larger_fn() -> Vec<&'static str> {
    vec![
        "33 Export",
        "Desperados",
        "Goldberg",
        "Gulder",
        "Heineken",
        "Star",
    ]
}

fn stout_fn() -> Vec<&'static str> {
    vec!["Legend", "Turbo King", "Williams"]
}

fn non_alcoholic_fn() -> Vec<&'static str> {
    vec!["Amstel Malta", "Fayrouz", "Maltina", "Malta Gold"]
}

fn write_vector_to_file(vector: Vec<&str>, file: &mut File, string: String) {
    for drink in vector {
        let total_string = string.clone() + drink + "\n";
        file.write_all(total_string.as_bytes())
            .expect("Unable to write to file");
    }
    file.write_all(b"\n").expect("Unable to write to file");
}
