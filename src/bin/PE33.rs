use std::time::Instant;

fn gcd(mut a: u64, mut b: u64) -> u64 {
	loop {
		if b == 0 { break a; }
		(a, b) = (b, a % b);
	}
}

fn main() {
	let time = Instant::now();
	
	let mut num_product = 1; 
	let mut den_product = 1;
	
	for n in 10..100 {
		for d in n+1..100 {
		
			let n_tens = n / 10;
			let n_units = n % 10;

			let d_tens = d / 10;
			let d_units = d % 10; 

			if n_units == d_tens && n_units != 0 && d_tens != 0
				&& n_tens * d == d_units * n
			{
				num_product *= n_tens;
				den_product *= d_units; 						
			}
		}
	}
	
	assert_eq!(den_product / gcd(num_product, den_product), 100);
	println!("{:?}", time.elapsed());
}
