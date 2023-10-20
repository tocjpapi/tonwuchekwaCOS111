fn main() {
    let p = 520000000.0;
    let r = 10.0;
    let n = 5.0;

    let a = p * (1.0 + r/100.0) * n;
    let ci = a - p;

    println!("The compound interest is {}", ci);
} 