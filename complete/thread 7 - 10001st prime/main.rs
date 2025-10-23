fn is_prime(n: u64) -> bool {
	if n < 2 { return false; }
	if n % 2 == 0 { return n == 2; }

  // i was messing around with Rust's ranges here
	(3u64..)
		.step_by(2)
		.take_while(|&x| x <= n / x )
		.all(|x| n % x != 0)
}

fn main() {
	let mut primes = (2u64..)
		.filter(|x| is_prime(*x));
  // unwrap because .nth() returns an Option 
	println!("{}", primes.nth(10000).unwrap());
}
