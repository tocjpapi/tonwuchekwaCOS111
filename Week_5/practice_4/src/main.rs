use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

     println!("comment vous appelez vous ?");
     io::stdin().read_line(&mut input1).expect("ce n'est pas une chaîne de caractères valide");

     println!("Quel est ton age ?");
     io::stdin().read_line(&mut input2).expect("ce n'est pas une chaîne de caractères valide");
     let age:i32 = input2.trim().parse().expect("ce n'est pas un numéro valide");

     if age >= 18 {
       println!("beinvenue sur la fête! {}",input1);
     }
    else {
        println!("Oups, vous n'avez pas l'âge requis pour participer à cette fête {}",input1);
    }
}
