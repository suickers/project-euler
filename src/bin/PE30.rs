fn pow_check(n: u64, e: u32) -> bool {
	if n == 1 { return false; }
	let s = n.to_string();
	n == s.chars().map(|x| (x as u64 - 0x30).pow(e)).sum()
}

fn main() {
	let e = 5;
	let lim = (9u64.pow(e))*6;
	let sum: u64 = (1..lim).filter(|&x| pow_check(x, e)).sum();
	
	assert_eq!(sum, 443839);
}
