use num::bigint::BigUint;

fn main() {
	let mut sum = BigUint::from(0usize);

	for i in 1..=1000 {
		sum += BigUint::from(i).pow(i);
	}

	let sumstr = sum.to_string();
	let len = sumstr.len();
	let ans = &sumstr[len-10..];
	
	assert_eq!(ans.parse::<usize>().unwrap_or(0), 9110846700);
}
