use project_euler::primes::is_prime;

fn main() {
	let n: u64 = 600851475143;
	let rootn: u64 = n.isqrt();
	let mut f = 0;

	for x in (1..=rootn).rev() {
		if !is_prime(x) { continue; }
		if n % x == 0 { f = x; break; }
	}

	assert_eq!(6857, f);
}
