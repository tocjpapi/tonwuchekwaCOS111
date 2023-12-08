use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let mut file = OpenOptions::new().append(true).open("data:txt").expect("Error opening");
    file.write_all("Hello Class".as_bytes()).expect("Error writing");

    println!("file append success");
}
