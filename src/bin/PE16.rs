use num_bigint::BigUint;
use num_traits::Pow;

fn main() {
	let n: BigUint = BigUint::from(2u32).pow(1000u32);
	let s: u32 = n
		.to_string()
		.chars()
		.map(|c| c.to_digit(10).unwrap())
		.sum();

		assert_eq!(s, 1366);
}
