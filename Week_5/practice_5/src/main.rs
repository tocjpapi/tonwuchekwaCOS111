//Taking your idea of telling us to be creative seriously, this whole project is in french

use std::io;

fn main() {

    let mut input1 = String ::new();
    println!("saisissez votre hauteur");
    io::stdin().read_line(&mut input1).expect("C'est pas un chaine de caracteres valide");
    let height:f32 = input1.trim().parse().expect("C'est pas un numero valide");

    if height >= 150.0 && height <= 170.0{
        println!("Vous etes de taille moyenne !");

     }
     else if height > 170.0 && height <= 195.0{
        println!("Vous etes grand !");
     }
     else if height < 150.0 && height > 100.0 {
        println!("Desole, vous etes un nain");
     }

     else {
         println!("Hauteur Anomale");
     }
}
