use std::time::Instant;

fn pent(n: usize) -> usize {
	(n * ((3 * n) - 1)) / 2
}

fn is_square(n: usize) -> bool {
    let root = n.isqrt();
    root * root == n 
}

fn is_pent(n: usize) -> bool {
    is_square((24*n) + 1) && ((24*n) + 1).isqrt() % 6 == 5
}

fn main() {

	let time = Instant::now(); 
	
	let mut ans = 0; 

	'outer: for d in 1.. {
		if !is_pent(d) { continue; }

		for i in 1..=1020 {
			let pk = pent(i); 
			let pj = pk + d; 

			if is_pent(pj) && is_pent(pj + pk) {
				ans = d; 
				break 'outer; 
			}
		}
	}
	
	assert_eq!(ans, 5482660);
	println!("{:?}", time.elapsed());
}
