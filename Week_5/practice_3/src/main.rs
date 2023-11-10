use std::io;

fn main() {
    let mut input1 = String ::new();
    let mut input2 = String ::new();


    println!("Enter{} ","base" );
    io::stdin().read_line(&mut input1).expect("not a valid string");
    let base:f32 = input1.trim().parse().expect("not a valid number");


    println!("Enter height");
    io::stdin().read_line(&mut input2).expect("not a valid string");
    let height:f32 = input2.trim().parse().expect("not a valid number");


    if base > 0.0{
        let area:f32 = (base * height) / 2.0;
        println!("the area of the triangle is {}",area );
    }
    else {
        println!("please end this program and try a number that's greater than 0");
    }


  
}
