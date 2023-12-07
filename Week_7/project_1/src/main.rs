use std::io;

fn trapezium() {
    println!("What's the height?");
    let mut height = String ::new();
    io::stdin().read_line(&mut height).expect("Not a string");
    let height:i32 = height.trim().parse().expect("Not a number");
    
    println!("What's the first base");
    let mut base1 = String ::new();
    io::stdin().read_line(&mut base1).expect("Not a string");
    let base1:i32 = base1.trim().parse().expect("Not a number");

    println!("What's the second base");
    let mut base2 = String ::new();
    io::stdin().read_line(&mut base2).expect("Not a string");
    let base2:i32 = base2.trim().parse().expect("Not a number");

    let area = (base1 + base2) * height / 2;

    println!("The area is {}", area);
}

fn rhombus() {
    println!("What's the first diagonal?");
    let mut height1 = String ::new();
    io::stdin().read_line(&mut height1).expect("Not a string");
    let height1:i32 = height1.trim().parse().expect("Not a number");
    
    println!("What's the second diagonal");
    let mut base = String ::new();
    io::stdin().read_line(&mut base).expect("Not a string");
    let base:i32 = base.trim().parse().expect("Not a number");

    let area2 = (base * height1) / 2;
    println!("The area is {}", area2);
    
}

fn parallelogram(){
    println!("What's the altitude?");
    let mut altitude = String ::new();
    io::stdin().read_line(&mut altitude).expect("Not a string");
    let altitude:i32 = altitude.trim().parse().expect("Not a number");
    
    println!("What's the second diagonal");
    let mut base2 = String ::new();
    io::stdin().read_line(&mut base2).expect("Not a string");
    let base2:i32 = base2.trim().parse().expect("Not a number");

    let area2 = base2 * altitude;
    println!("The area is {}", area2);
}

fn cube(){
   
    println!("What's the width?");
    let mut width = String ::new();
    io::stdin().read_line(&mut width).expect("Not a string");
    let width:i32 = width.trim().parse().expect("Not a number");

    let area3 = 6*width.pow(2);
    println!("The area is {}", area3);
}

fn cylinder(){
    println!("What's the radius?");
    let mut radius = String ::new();
    io::stdin().read_line(&mut radius).expect("Not a string");
    let radius:i32 = radius.trim().parse().expect("Not a number");

    println!("What's the height?");
    let mut tall = String ::new();
    io::stdin().read_line(&mut tall).expect("Not a string");
    let tall:i32 = tall.trim().parse().expect("Not a number");
    

    let area4 = 22/7 * radius * radius * tall;
    println!("The area is {}", area4);
}

fn main() {
    println!("What do you want to calculate?\n1. Trapezium\n2. Rhombus\n3. Parallelogram\n4. Cube\n5. Cylinder");
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice = choice.trim();  // Trim to remove leading and trailing whitespaces

    if choice == "1" {
        trapezium();
    } else if choice == "2" {
        rhombus();
    } else if choice == "3" {
        parallelogram();
    } else if choice == "4" {
        cube();
    } else if choice == "5" {
        cylinder();
    } else {
        println!("Invalid choice");
    }
}
