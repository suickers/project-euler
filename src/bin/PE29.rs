use num::bigint::BigUint;
use std::collections::HashSet;

fn main() {
	let mut set: HashSet<BigUint> = HashSet::new();
	let mut a = BigUint::from(2u64);

	for _ in 2..=100 {
		for b in 2..=100 {
			set.insert(a.pow(b));	
		}
		a += BigUint::from(1u64);
	}
	let answer = set.len();

	assert_eq!(answer, 9183);
}
