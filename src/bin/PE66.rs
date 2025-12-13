
fn print_cf(mut x: usize, mut y: usize) {
	loop {
		let q = x / y;
		let r = x % y;

		println!("{}", q);
		if r == 0 { break; }
		
		(x, y) = (y, r);
	}
}

fn main() {
	print_cf(43, 19);	
}
 
