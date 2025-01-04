use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data.txt").expect(
        "cannot open file"
    );

    file.write_all(b"Hello Class\n").expect("write failed");
    file.write_all(b"This is the appendage to the document.\n").expect("write failed");
    println!("file append success");
}
