use std::fs;

fn main() {
    fs::remove_file("data.txt").expect("couldn't remove data.txt");
    println!("file is removed");
}