use std::collections::HashMap;

fn length(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
	if let Some(&v) = memo.get(&n) {
		return v; 
	}
	let next = if n % 2 == 0 { n / 2 } else { (3*n) + 1 };
	let v = 1 + length(next, memo);
	memo.insert(n, v);
	v
}

fn main() {
	let mut max_n = 0;
	let mut max_s = 0;
	let mut memo = HashMap::new();
	memo.insert(1, 0);
	
	for i in 1..1_000_000 {
		let s = length(i, &mut memo);

		if s > max_s {
			max_s = s;
			max_n = i;
		}
	}
	println!("{}", max_n);
}
