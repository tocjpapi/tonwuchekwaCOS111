use std::io;

fn main() {
    
    let mut y = String ::new();
    println!("What time's table do you wan't to generate?");
    io::stdin().read_line(&mut y).expect("Not a valid string");
    let y:i32 = y.trim().parse().expect("Not a number");
    
    let mut g = String ::new();
    println!("It's to be multiplied how many times?");
    io::stdin().read_line(&mut g).expect("Not a valid string");
    let g:i32 = g.trim().parse().expect("Not a number");

    for x in 1..g + 1{
        let times = y * x;

        println!("{} x {} = {}", y, x, times);

        if x > g + 1  {
            break;
        }
    }

}
