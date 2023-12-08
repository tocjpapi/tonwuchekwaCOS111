use std::io::Write;i

fn main() {
    println!("**********PAU-SMIS**********");

    let students = ["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Blanca Edemo"];
    let matric_numbers = ["ACC10211111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let departments = ["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let levels = ["300", "100", "200", "200", "100"];

    let mut file = std::fs::File::create("PAU-SMIS.txt").expect("Couldn't create");

    writeln!(file, "**********PAU-SMIS**********").expect("Write failed");

    for i in 0..students.len() {
        writeln!(
            file,
            "Student Names: {}\nMatric Number: {}\nDepartment: {}\nLevel: {}\n",
            students[i], matric_numbers[i], departments[i], levels[i]
        )
        .expect("Write failed");
    }

    println!("Nous avons fini Merci!\n");
}
