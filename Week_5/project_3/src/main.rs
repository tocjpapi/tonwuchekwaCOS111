use std::io;

fn main() {
    loop {
        println!("Sur La Menu");
        println!("P = Poundo Yam/Edikaiko Soup\t- N3,200");
        println!("F = Fried Rice & Chicken\t- N3,000");
        println!("A = Amala & Ewedu Soup\t\t- N2,500");
        println!("E = Eba & Egusi Soup\t\t- N2,000");
        println!("W = White Rice & Stew\t\t- N2,500");

        println!("Qu'est que vous souhaitez manger ? (P/F/A/E/W):");
        let mut food_type = String::new();
        io::stdin().read_line(&mut food_type).expect("Erreur");
        let food_type = match food_type.trim().to_uppercase().chars().next() {
            Some(c) => c.to_string(),
            None => {
                println!("Une lettre seulement...");
                return;
            }
        };

        println!("La quantité ?:");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).expect("Invalide");
        let quantity: u32 = quantity.trim().parse().expect("Encore?");

        let total_charges = match food_type.as_str() {
            "P" => 3200 * quantity,
            "F" => 3000 * quantity,
            "A" => 2500 * quantity,
            "E" => 2000 * quantity,
            "W" => 2500 * quantity,
            _ => {
                println!("Type d'aliment non valide");
                return;
            }
        };

        let discounted_charges = if total_charges > 10000 {
            total_charges - (5/100 * total_charges)
        } else {
            total_charges
        };

        println!("Total des frais pour cette commande: N{}", discounted_charges);

        // Ask if the user wants to place another order
        println!("Souhaitez-vous passer une autre commande ? (Y/N):");
        let mut answer = String::new();
        io::stdin().read_line(&mut answer).expect("Echoue");

        if answer.trim().to_lowercase() != "y" {
           
            break;
        }
    }
}