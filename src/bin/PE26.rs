use std::collections::HashMap; 

fn cycle_length(b: usize) -> usize {
	let mut hash: HashMap<usize, usize> = HashMap::new(); 
	let mut a = 1; 
	let mut t = 0; 

	loop {
		hash.insert(a, t);
		a = a % b * 10;
		t += 1;
		
		if hash.contains_key(&a) {
			break;
		}
	}
	t - hash[&a]
}

fn main() {
	let mut max_d = 0; 
	let mut max_len = 0;
	
	for d in 1..1000 {
		let len = cycle_length(d);

		if len > max_len {
			max_len = len;
			max_d = d;
		}
	}
	assert_eq!(max_d, 983);
}
