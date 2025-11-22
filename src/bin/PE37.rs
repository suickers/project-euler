use project_euler::primes::*;
use std::time::Instant;
use std::iter::successors;

fn truncate(n: usize, sieve: &[bool]) -> bool {
	let len = n.ilog10() + 1;
	
	let exp1 = successors(Some(n), |x| {
		if *x >= 10 { Some(x / 10) } else { None }
	})
		.all(|x| sieve[x]);

	let exp2 = (1..len)
		.rev()
		.map(|x| n % 10usize.pow(x))
		.all(|x| sieve[x]);

	exp1 && exp2
}

fn main() {
	let time = Instant::now();
	let limit = 10usize.pow(6);
	let mut sum = 0;
	let sieve = sieve(limit);
	
	for i in 8..=limit {
		if truncate(i, &sieve) {
			sum += i;
		}
	}

	assert_eq!(sum, 748317);
	println!("{:?}", time.elapsed());
}
