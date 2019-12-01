use std::fs::File;
use std::io::BufReader;

fn main() {
    println!("Hello, world!");
}

fn readData() -> String {
    let mut reader = BufReader::new(File::open("data.txt"));
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return String::default();
}
