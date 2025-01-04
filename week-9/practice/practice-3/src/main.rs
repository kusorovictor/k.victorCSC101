use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("");
    println!("file is removed");
}
