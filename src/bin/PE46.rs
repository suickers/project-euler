use std::time::Instant; 

use project_euler::primes::*;

fn is_square(n: usize) -> bool {
	let root = n.isqrt();

	root * root == n
}

fn is_property(n: usize) -> bool {
	Primes::new()
		.map(|p| p as usize)
		.take_while(|&p| p < n)
		.any(|p| is_square((n - p) / 2))
}

fn main() {
	let time = Instant::now(); 

	let ans = (33..).step_by(2)
		.filter(|&x| !is_prime(x))
		.find(|&x| !is_property(x))
		.unwrap_or(0);

	assert_eq!(ans, 5777);

	println!("{:?}", time.elapsed());
}
