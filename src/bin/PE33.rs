fn checker(n: usize, x: usize, y: usize) -> bool {
	let fraction = ((10*n)+x) / ((10*n)+y);
	if fraction != x / y {
		return false;
	} 

	let fraction = ((10*x)+n) / ((10*n)+y);
	if fraction != x / y {
		return false;
	} 

	let fraction = ((10*x)+n) / ((10*y)+n);
	if fraction != x / y {
		return false;
	} 

	let fraction = ((10*n)+x) / ((10*y)+n);
	if fraction != x / y {
		return false;
	} 
	true
	
}

fn main() {
	for i in 1..=99 {
		for j in 1..99 {
			for k in 1..99 {
				if checker(k, i, j) {
					println!("{}, {}, {}", i, j, k);
				}
			}
		}
	}
}
