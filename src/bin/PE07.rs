use project_euler::primes::is_prime;

fn main() {
	let mut primes = (2u64..)
		.filter(|x| is_prime(*x));

	assert_eq!(primes.nth(10000).unwrap(), 104743);
}
