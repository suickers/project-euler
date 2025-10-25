fn is_prime(n: u64) -> bool {
	if n < 2 { return false; }
	if n % 2 == 0 { return n == 2; }

	let mut v = 3;
	while v < n / v {
		if n % v == 0 { return false; }
		v += 2; 
	}
	true
}

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
