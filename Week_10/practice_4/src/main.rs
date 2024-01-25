fn main() {
    // a list of numbers
    let v = vec![15, 25, 35, 45, 55];
    
    // Print the original vector before calling the function
   

    // Pass a reference to the vector
    print_vector(&v);

    // Accessing an element by index works because we're not borrowing the whole vector
    println!("{}", v[0]);
}

fn print_vector(x: &[i32]) {
    println!("Inside print_vector function: {:?}", x);
}
