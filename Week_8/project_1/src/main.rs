use std::io;


fn main() {
loop {   
    println!("What type of public servant are you?
            1. Lawyer
            2. Office Administrator
            3. Academic
            4.teacher");
    let mut servant_type = String::new();
    io::stdin().read_line(&mut servant_type).expect("Error reading");
    let servant_type: String = servant_type.trim().to_lowercase();

    if servant_type == "lawyer"{
        lawyer();
        break;
    }
    else if servant_type == "office administrator" {
        office();
        break;
    }
    else if servant_type == "academic" {
        academic();
        break;
    }
    else if servant_type == "teacher" {
        teacher();
        break;
}
    else {
        println!("Please read the questions correctly");
    }
} 
}

fn lawyer(){
    let levels = vec![ 
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
       "EL2 10-13",
        "SES",
     ];
    println!("What type of lawyer are you?");
let mut lawyer_type = String::new();
io::stdin().read_line(&mut lawyer_type).expect("Error reading");

    println!("How many years of experience do you have?");
let mut lawyer_experience = String::new();
io::stdin().read_line(&mut lawyer_experience).expect("Error reading");
let lawyer_experience: u32 = lawyer_experience.trim().parse().expect("Not an integer");


if lawyer_experience >= 0 && lawyer_experience <= 2 {
    println!("Your level is {:?}", levels[0])
}
else if lawyer_experience >= 3 && lawyer_experience <= 5 {
    println!("Your level is {:?}", levels[1])
}
else if lawyer_experience > 5 && lawyer_experience <= 8 {
    println!("Your level is {:?}", levels[2])
}
else if lawyer_experience > 8 && lawyer_experience <= 10 {
    println!("Your level is {:?}", levels[3])
}
else if lawyer_experience > 10 && lawyer_experience <= 13 {
    println!("Your level is {:?}", levels[4])
}
else if lawyer_experience > 13 {
    println!("Your level is {:?}", levels[5])
}
}



fn office(){
    let levels = vec![ 
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
       "EL2 10-13",
        "SES",
     ];
    println!("What type of Office administrator are you?");
let mut office_type = String::new();
io::stdin().read_line(&mut office_type).expect("Error reading");

    println!("How many years of experience do you have?");
let mut office_experience = String::new();
io::stdin().read_line(&mut office_experience).expect("Error reading");
let office_experience: u32 = office_experience.trim().parse().expect("Not an integer");


if office_experience >= 0 && office_experience <= 2 {
    println!("Your level is {:?}", levels[0])
}
else if office_experience >= 3 && office_experience <= 5 {
    println!("Your level is {:?}", levels[1])
}
else if office_experience > 5 && office_experience <= 8 {
    println!("Your level is {:?}", levels[2])
}
else if office_experience > 8 && office_experience <= 10 {
    println!("Your level is {:?}", levels[3])
}
else if office_experience > 10 && office_experience <= 13 {
    println!("Your level is {:?}", levels[4])
}
else if office_experience > 13 {
    println!("Your level is {:?}", levels[5])
}
}

fn academic(){
    let levels = vec![ 
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
       "EL2 10-13",
        "SES",
     ];
    println!("What type of academic are you?");
let mut office_type = String::new();
io::stdin().read_line(&mut office_type).expect("Error reading");

    println!("How many years of experience do you have?");
let mut office_experience = String::new();
io::stdin().read_line(&mut office_experience).expect("Error reading");
let office_experience: u32 = office_experience.trim().parse().expect("Not an integer");


if office_experience >= 0 && office_experience <= 2 {
    println!("Your level is {:?}", levels[0])
}
else if office_experience >= 3 && office_experience <= 5 {
    println!("Your level is {:?}", levels[1])
}
else if office_experience > 5 && office_experience <= 8 {
    println!("Your level is {:?}", levels[2])
}
else if office_experience > 8 && office_experience <= 10 {
    println!("Your level is {:?}", levels[3])
}
else if office_experience > 10 && office_experience <= 13 {
    println!("Your level is {:?}", levels[4])
}
else if office_experience > 13 {
    println!("Your level is {:?}", levels[5])
}
}

fn teacher(){
    let levels = vec![ 
        "APS 1-2",
        "APS 3-5",
        "APS 5-8",
        "EL1 8-10",
       "EL2 10-13",
        "SES",
     ];
    println!("What type of teacher are you?");
let mut office_type = String::new();
io::stdin().read_line(&mut office_type).expect("Error reading");

    println!("How many years of experience do you have?");
let mut office_experience = String::new();
io::stdin().read_line(&mut office_experience).expect("Error reading");
let office_experience: u32 = office_experience.trim().parse().expect("Not an integer");


if office_experience >= 0 && office_experience <= 2 {
    println!("Your level is {:?}", levels[0])
}
else if office_experience >= 3 && office_experience <= 5 {
    println!("Your level is {:?}", levels[1])
}
else if office_experience > 5 && office_experience <= 8 {
    println!("Your level is {:?}", levels[2])
}
else if office_experience > 8 && office_experience <= 10 {
    println!("Your level is {:?}", levels[3])
}
else if office_experience > 10 && office_experience <= 13 {
    println!("Your level is {:?}", levels[4])
}
else if office_experience > 13 {
    println!("Your level is {:?}", levels[5])
}
}