fn truncate(mut n: usize) -> Vec<usize> {
	let len = n.ilog10() + 1; 

	let mut vec = Vec::new(); 

	for _ in 1..=len {
		vec.push(n);
		n /= 10;
	}	
	vec
}

fn main() {
	println!("{:?}", truncate(3797));
}
