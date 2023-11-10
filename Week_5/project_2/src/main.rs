use std::io;

fn main() {
    let mut ag = String ::new();
    println!("Quelle age avez-vous?");
    io::stdin().read_line(&mut ag).expect("Invalide");
    let age:u32 = ag.trim().parse().expect("Invalide");

     


    if age > 40 { println!(" votre incitation est de N1 560 000");

    }
    else if age >= 30 && age < 40 { println!("votre incitation est de N1 480 000");

}
    else if age < 28 {
       println!(" votre incitation est de N1 300 000");
}
       println!("avez-vous de l'expérience ? Y ou N");
     let mut multi = String ::new();
     io::stdin().read_line(&mut multi).expect("n'a pas tout à fait fonctionné");
     let answer: bool = match multi.trim().to_lowercase().as_str(){
        "y" => true,
        "n" => false, 
          _ => {
            println!("Reesayez avec Y our N"); 
        return;
     }
     
     };
        
     if answer {
        println!("Votre incintation est correcte! stp fermez cette Fenêtre");
     }
     else {
        println!("Vous etes Inexpérimenté donc votre incintation est de N100 000");
     }

    }