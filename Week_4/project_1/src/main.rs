use std::io;

fn main() {
    let mut distance=String ::new();
    let mut time=String ::new();
    
    println!("What's the distance your using to calculate ?");
    io::stdin().read_line(&mut distance).expect("Invalid");
    let a:f32 = distance.trim().parse().expect("Still invalid");

    println!("What's the time your using to calculate ?");
    io::stdin().read_line(&mut time).expect("Invalid");
    let b:f32 = time.trim().parse().expect("Still invalid");

    let speed:f32 = a/b;

    println!("The speed is {:.2}", speed);

    println!("would you like to do this again for a different set of values? y/n");
    let mut choice = String ::new();
    io::stdin().read_line(&mut choice).expect("Didn't quite work");
    let answer = match choice.trim().to_lowercase().as_str(){
        "y" => true,
        "n" => false,
                _ =>{
            println!("Wrong input, please try 'y' or 'n'.");
            return;
        }
    };
    if answer{
    println!("please re-execute the program to try new values" );
}
     else {
    println!("You can now end this session thanks!" );

}
}
