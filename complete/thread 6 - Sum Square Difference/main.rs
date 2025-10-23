fn main() {
	let (sum, sumsq) = (1..=100)
		.fold((0, 0), |(s, ssq), x| (s + x, ssq + x*x));
	print!("{}", (sum*sum) - sumsq);
}
