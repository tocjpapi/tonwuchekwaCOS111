use std::io;

fn main() {
    println!("Enter the number of siblings:");
    let num_sibs: u32 = get_input().trim().parse().expect("Invalid input");

    let mut sibs_data = Vec::new();

    for s in 1..=num_sibs {
        println!("Enter details for Sibling {}: ", s);

        let name = get_input("Enter first name:");
        let age: u32 = get_input("Enter age:").trim().parse().expect("Invalid input");

        let sib_info = if age > 18 {
            if get_input("Is the sibling married? yes/no:") == "yes" {
                format!("Marital Status: Married, Offspring: {}, City: {}", get_input("How many offsprings?"), get_input("What city does the family live in?"))
            } else {
                if get_input("Is the sibling a student or a worker?") == "student" {
                    format!("Marital Status: Single, Status: Student, University: {}, Course: {}", get_input("Enter university:"), get_input("Enter course:"))
                } else {
                    "Marital Status: Single, Status: Worker".to_string()
                }
            }
        } else {
            if get_input("Do you have WAEC?") == "yes" {
                format!("WAEC Status: Yes, Secondary School: {}", get_input("Enter secondary school attended:"))
            } else {
                format!("WAEC Status: No, Current Class: {}", get_input("Enter current class:"))
            }
        };

        sibs_data.push(format!("Sibling {}: Name: {}, Age: {}", s, name, age));
        sibs_data.push(sib_info);
    }

    println!("Sibling Data:");
    for sib in &sibs_data {
        println!("{}", sib);
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}



