use std::io;

fn main() {
    println!("saisir la borne inférieure");
    let mut input1 = String ::new();
    io::stdin().read_line(&mut input1).expect("nous n'avons pu lire l'entrée");
    let lower_bound:i32 = input1.trim().parse().expect("n'a pas lu l'entrée");

    println!("saisir la borne supérieure");
    let mut input2 = String ::new();
    io::stdin().read_line(&mut input2).expect("nous n'avons pu lire l'entrée");
    let upper_bound:i32 = input2.trim().parse().expect("n'a pas lu l'entrée");
    for x in lower_bound..upper_bound{
        println!("La niveau de comptage est {}",x);
    }
}
