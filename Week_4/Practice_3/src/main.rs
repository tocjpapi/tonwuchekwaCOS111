fn main() {
    let name1 = "Ayomide Adesokan";
    println!("My name is {}", name1);

    let name2 = name1.replace("Ayomide","Adebare");
    println!("You can also call me {}", name2);
    let faculty = "Faculty of science and Technology";


    let school = faculty.replace("Faculty", "School");
    println!("I'm a student of the {}",school );
}
