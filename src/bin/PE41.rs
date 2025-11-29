use std::time::Instant;
use std::iter::successors; 
use project_euler::primes::*;

fn is_pandigital(x: usize, d: usize) -> bool {
    let mut arr = vec![true; d + 1];
    arr[0] = false;
    
    if length_of(x) != d { return false; }

    for n in digits_of(x) {
    	if n > d { return false; }
        arr[n] = false;
    }

    arr == vec![false; d + 1]
}

fn digits_of(a: usize) -> impl Iterator<Item = usize> {
    successors(Some(a), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
}

fn length_of(x: usize) -> usize {
    (x.ilog10() + 1) as usize
}

fn main() {
	let time = Instant::now();
	
	let ans = (1234..=7654321).rev()
		.filter(|x| is_prime(*x))
		.find(|x| {
			let n = length_of(*x);
			is_pandigital(*x, n)
		})
		.unwrap();

	assert_eq!(ans, 7652413);
	println!("{:?}", time.elapsed());
}
