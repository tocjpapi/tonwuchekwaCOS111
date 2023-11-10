use std::io;

fn main() {
    let mut aa = String ::new();
    let mut bb = String ::new();
    let mut cc = String ::new();

    
    

    println!("en utilisant le format d'équation quadratique 'ax^2 + bx + c' saisissez 'a'");
    io::stdin().read_line(&mut aa).expect("Invalide!");
    let a:i32 = aa.trim().parse().expect("Invalide!");

    println!("en utilisant le format d'équation quadratique 'ax^2 + bx + c' saisissez 'b'");
    io::stdin().read_line(&mut bb).expect("Invalide!");
    let b:i32 = bb.trim().parse().expect("Invalide!");

    println!("en utilisant le format d'équation quadratique 'ax^2 + bx + c' saisissez 'c'");
    io::stdin().read_line(&mut cc).expect("Invalide!");
    let c:i32 = cc.trim().parse().expect("Invalide!");



    let ba = b.pow(2) - 4 * a * c;

    if ba == 0{
        println!("Cette équation quadratique a deux racines égales identiques");
    }
    else if ba > 0{
        println!("Cette équation quadratique a deux racines égales identiques");
    }
    else{
        println!("Il semble s'agir d'une équation purement imaginaire");
    }
}
