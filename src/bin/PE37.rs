use project_euler::primes::sieve;
use std::time::Instant;

fn truncate(n: usize) -> Vec<usize> {
	let len = n.ilog10() + 1; 
	let mut en = n;
	let mut vec = Vec::new(); 

	for _ in 1..=len {
		vec.push(en);
		en /= 10;
	}	
	en = n;

	for i in (0..len).rev() {
		vec.push(en);
		en %= 10usize.pow(i);
	}
	vec
	
}

fn main() {
	let time = Instant::now();
	
	let mut sum = 0;
	let limit = 10usize.pow(6);
	let sieve = sieve(limit);
	for i in 8..=limit {
		if truncate(i).into_iter().all(|x| sieve[x]) {
			sum += i;
		}
	}

	println!("{}", sum);
	println!("{:?}", time.elapsed());
}
