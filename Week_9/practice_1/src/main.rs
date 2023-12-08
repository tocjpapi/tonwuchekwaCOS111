use std::io::Write;

fn main() {
    let announce = "Week 9 - Rust File Input & Output";
    let dept = "Department of Computer Science";

    let mut file = std::fs::File::create("data.txt").expect("Error Creating File");
    file.write_all("Welcome to Rust Programming\n".as_bytes()).expect("Error Writing");
    
    file.write_all(announce.as_bytes()).expect("Error Writing");
    file.write_all(dept.as_bytes()).expect("Error Writing");


    println!("\n It's been successfully created.")
}
