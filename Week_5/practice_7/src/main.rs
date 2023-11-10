use std::io;

fn main() {
    println!("entrer un numéro");
    let mut input1 = String ::new();
    io::stdin().read_line(&mut input1).expect("n'a pas lu l'entrée");
    let mut num:i32 = input1.trim().parse().expect("Erreur de conversion du numéro. Réessayez.");


    while num < 10{
        println!("La valeur de la boucle interne est {}", num);
        num+=1;
    }
    println!("la valeur de la boucle extérieure est {}", num)
}
