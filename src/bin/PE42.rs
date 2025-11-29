use std::{fs::File, io};
use std::time::Instant;

fn is_square(n: usize) -> bool {
	let root = n.isqrt(); 
	root * root == n
}

fn is_triangle(n: usize) -> bool {
	is_square(8 * n + 1)
}


fn main() -> io::Result<()> {
	let time = Instant::now();
	
	let file = File::open("data/0042_words.txt")?;

	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_reader(file);

	let record = rdr.records().next().unwrap()?;

	let out = record.iter()
		.map(|w| {
			w.chars()
				.map(|c| c as u8 - b'A' + 1)
				.map(|n| n as usize)
				.sum::<usize>()
		})
		.filter(|x| is_triangle(*x))
		.count();

	assert_eq!(out, 162);
	println!("{:?}", time.elapsed());

	Ok(())
}
