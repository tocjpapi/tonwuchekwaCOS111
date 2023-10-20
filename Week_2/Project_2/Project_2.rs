fn main() {
	let toshiba = 2.0 * 450000.0;
	let mac = 1500000.0;
	let hp = 3.0 * 750000.0;
	let dell = 3.0 * 2850000.0;
	let acer = 250000.0;
	let sum = acer + dell + toshiba + mac + hp;
	let freq = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;
	let avg = sum / freq;

	println!("the average is {} and the sum is {}", avg, sum);
}