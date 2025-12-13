use std::time::Instant;

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

	for i in 144.. {
		let hex = i * (2*i - 1);
		
		if is_pent(hex) {
			ans = hex; 
			break;
		}
	}
	
	assert_eq!(ans, 1533776805);
	println!("{:?}", time.elapsed()); 
	

}
