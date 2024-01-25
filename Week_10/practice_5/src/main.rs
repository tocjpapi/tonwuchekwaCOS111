fn main() {
    // a list of numbers
    let x = vec![100, 200, 300];
    borrow_vector(&x); // passing reference

    println!("Printing the value from main() x[0]={}", x[0]);
}

fn borrow_vector(z: &Vec<i32>) {
    println!("***********************");
    println!("Inside borrow_vector function: {:?}", z);
    println!("---------------------------");
}
