use std::iter::successors;
use project_euler::primes::{Primes, sieve}; 
use std::time::Instant;

fn digits_of(a: usize) -> impl Iterator<Item = usize> {
    successors(Some(a), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
}

fn are_perms(a: usize, b: usize, c: usize) -> bool {
	let mut da = digits_of(a).collect::<Vec<_>>(); 
	let mut db = digits_of(b).collect::<Vec<_>>(); 
	let mut dc = digits_of(c).collect::<Vec<_>>(); 
	
	da.sort();
	db.sort(); 
	dc.sort();
	
	da == db && db == dc 
}

fn concat(a: usize, b: usize, c: usize) -> usize {
    let n1 = b.ilog10() + 1;
    let n2 = c.ilog10() + 1;

    (a * 10usize.pow(n1) + b) * 10usize.pow(n2) + c
}

fn condition(p: usize, p1: usize, p2: usize, sieve: &[bool]) -> bool {
	sieve[p1] && sieve[p2] && are_perms(p, p1, p2) && p != 1487
}

fn main() {

	let time = Instant::now(); 

	let sieve = sieve(10000);
	
	let ans = Primes::new()
		.map(|p| p as usize)
		.filter(|&p| (1000..=9999).contains(&p))
		.flat_map(|p| {
			(1..=4500)
				.take_while(move |&d| p + 2*d < 10000)
				.map(move |d| (p, d))
				.filter_map(|(p, d)| {
					let p1 = p + d;
					let p2 = p + 2*d;
					
					if condition(p, p1, p2, &sieve) {
						Some((p, p1, p2))
					} else {
						None
					}
				})
		})
		.next()
		.map(|(p, p1, p2)| concat(p, p1, p2))
		.unwrap();

	assert_eq!(ans, 296962999629);

	println!("{:?}", time.elapsed());
		
}
