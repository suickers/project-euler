use std::time::Instant;

const PRIMES: &[usize] = &[2, 3, 5, 7, 11, 13, 17];

struct Perm {
	v: Vec<usize>,
	is_done: bool
}

impl Perm {
	fn new(v: Vec<usize>) -> Self {
		Self {v, is_done: false}
	}
}

impl Iterator for Perm {
	type Item = Vec<usize>;

	fn next(&mut self) -> Option<Self::Item> {
		let c = self.v.clone();
		
		if self.is_done {
			return None; 
		}
		
		let n = self.v.len(); 

		if n < 2 { 
			self.is_done = true; 
			return Some(c);
		}

		let mut i = n - 2;
		while i > 0 && self.v[i] >= self.v[i + 1] {
			i = i.saturating_sub(1); 
		}

		if self.v[i] >= self.v[i+1] {
			self.is_done = true;
			return Some(c);
		}

		let mut j = n - 1;
		while j > i && self.v[j] <= self.v[i] {
			j = j.saturating_sub(1); 
		}

		self.v.swap(i, j);
		self.v[i+1..].reverse();
		
		Some(c)
	}
}

fn concat(a: usize, b: usize, c: usize) -> usize {
    (a * 10 + b) * 10 + c
}

fn is_property(v: &[usize]) -> bool {
    for i in 0..7 {
        let d1 = v[i + 1];
        let d2 = v[i + 2];
        let d3 = v[i + 3];
        
        let x = concat(d1, d2, d3);
        if !x.is_multiple_of(PRIMES[i]) { return false; }
    }
    
    true
}

fn main() {

	let time = Instant::now();

	let v = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

	let perm = Perm::new(v);

	let ans: usize = perm
		.filter(|v| is_property(v))
		.map(|v| {
			v.iter().fold(0, |acc, &x| acc * 10 + x)
		})
		.sum();

	assert_eq!(ans, 16695334890);
	println!("{:?}", time.elapsed());
}
