use std::io;

fn main() {
    loop {
       let mut count = 0;
       count+=1;
       let mut name = String ::new();
       println!("What's your name?");
       io::stdin().read_line(&mut name).expect("Use Letters");

       let mut email = String ::new();
       println!("What's your email?");
       io::stdin().read_line(&mut email).expect("Use Letters");

       let mut department = String ::new();
       println!("What's your department?");
       io::stdin().read_line(&mut department).expect("Use Letters");

       let mut state = String ::new();
       println!("Where are you from?");
       io::stdin().read_line(&mut state).expect("Use letters");

       let mut rep = String ::new();
       println!("Are you a class rep currently?\nIf yes press 1\n If no press 0");
       io::stdin().read_line(&mut rep).expect("Invalid");
       let rep:f32 = rep.trim().parse().expect("Didn't parse output");

       let mut level = String ::new();
       println!("What's your level?\nIf 100level press 1\nIf 200level press 2\nIf 300level press 3\nIf 400 level press 4 \nIf 500 level press 5\nall others should press 6");
       io::stdin().read_line(&mut level).expect("Invalid");
       let level:f32 = level.trim().parse().expect("Didn't parse output");
        
       let mut cgpa = String ::new();
       println!("What's your CGPA?");
       io::stdin().read_line(&mut cgpa).expect("Invalid");
       let cgpa:f32 = cgpa.trim().parse().expect("Didn't parse output");

       if level > 1.0 && cgpa > 4.0 && rep == 1.0 {
            println!(
                "Hey, {} contacted with {}, who attends {}, from {}.\nYou can vote.",
                name, email, department, state
            );
        } else {
            println!("Sorry, you are not eligible to vote");
        }
        if count == 150 {
            break;
        }
    }
}