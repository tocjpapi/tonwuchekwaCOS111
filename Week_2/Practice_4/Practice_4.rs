fn main()  {
	let p:f64 = 1000.0;
	let r:f64 = 1.0;
	let t:f64 = 2.0;

    // CI
    let a = p*(1.0 + (r / 100.0)) * t;
    let ci = a - p;
     
    println!("Amount is {}",a);
    println!("Compound interest is {}",ci);
}