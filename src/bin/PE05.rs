fn gcd(mut a: u64, mut b: u64) -> u64 {
	loop {
		if b == 0 { break a; }
		(a, b) = (b, a % b);
	}
}

fn lcm(a: u64, b: u64) -> u64 { a / gcd(a, b) * b }
	
fn main() {
	let result = (1..=20).fold(1, lcm);
	print!("{result}");
}
