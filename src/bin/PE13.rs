use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
	let f = File::open("../../data/euler13.txt")?;
	let reader = BufReader::new(f);

	let mut vec = Vec::new();

	for line in reader.lines() {
		vec.push(line?);
	}

	let sum: u128 = vec
		.iter()
		.map(|x| &x[..12])
		.map(|p| p.parse::<u128>().unwrap())
		.sum::<u128>()
		.to_string()[..10]
		.parse::<u128>()
		.unwrap();

	assert_eq!(5537376230, sum);
	Ok(())
}
