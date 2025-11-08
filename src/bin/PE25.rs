use project_euler::kibonacci::KBonacci;
use num::bigint::BigUint;

fn count_digits(n: &BigUint) -> usize {
	n.to_string().len()
}

fn main() {
	let fib = KBonacci::new(2)
		.enumerate(); 
		
	let mut ans = 0;

	for (idx, f) in fib {
		if count_digits(&f) >= 1000 {
			ans = idx;
			break;
		}
	}

	assert_eq!(ans, 4782);
}
