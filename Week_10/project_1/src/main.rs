// Define a struct to represent a laptop brand
struct Laptop {
    brand: String,
    price: u32,
}

// Implement methods for the Laptop struct
impl Laptop {
    // Calculate the total cost for a given quantity
    fn total_cost(&self, quantity: u32) -> u32 {
        self.price * quantity
    }
}

fn main() {
    // Define laptops for each brand
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        price: 650000,
    };

    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        price: 755000,
    };

    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        price: 550000,
    };

    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        price: 850000,
    };

    // Simulate a customer purchasing 3 laptops from each brand
    let quantity_per_brand = 3;

    // Calculate the total cost for each brand
    let total_cost_hp = hp_laptop.total_cost(quantity_per_brand);
    let total_cost_ibm = ibm_laptop.total_cost(quantity_per_brand);
    let total_cost_toshiba = toshiba_laptop.total_cost(quantity_per_brand);
    let total_cost_dell = dell_laptop.total_cost(quantity_per_brand);

    // Calculate the overall total cost
    let overall_total_cost =
        total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;

    // Print the results
    println!("Total cost for HP laptops: {}", total_cost_hp);
    println!("Total cost for IBM laptops: {}", total_cost_ibm);
    println!("Total cost for Toshiba laptops: {}", total_cost_toshiba);
    println!("Total cost for Dell laptops: {}", total_cost_dell);
    println!("Overall total cost: {}", overall_total_cost);
}
