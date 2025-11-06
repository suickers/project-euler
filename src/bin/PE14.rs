// naive recursive approach. trying to think how to memoize it. 

fn length(n: usize, count: usize) -> usize {
	if n == 1 { return count; }
	length(if n % 2 == 0 { n / 2 } else { (3*n) + 1 }, count + 1)
}

fn main() {
	let mut max_n = 0;
	let mut max_s = 0;

	for i in 1..1_000_000 {
		let s = length(i, 1);

		if s > max_s {
			max_s = s;
			max_n = i;
		}
	}
	println!("{}", max_n);
}
