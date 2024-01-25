fn main() {
    let v = vec![10, 20, 30];

    let v2 = display(v.clone());

    println!("In main {:?}", v2);
}

fn display(v: Vec<i32>) -> Vec<i32> {
    println!("inside display {:?}", v.clone());
    
    // return ownership back to the caller
    v
}
