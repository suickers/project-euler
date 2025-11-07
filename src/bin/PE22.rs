use std::{fs::File, io};

fn letter_score(c: char) -> u32 {
	(c as u8 & 0x1F) as u32
}

fn name_score(name: &str) -> u32 {
	name.chars().map(letter_score).sum()
}

fn main() -> io::Result<()> {

	let file = File::open("../../data/names.txt")?;
	
	let mut rdr = csv::ReaderBuilder::new()
		.has_headers(false)
		.from_reader(file);
		
	let record = rdr.records().next().unwrap()?;
	
	let mut names: Vec<String> = record
		.iter()
		.map(|s| s.to_string())
		.collect();

	names.sort();

	let mut sum = 0;
	
	for (idx, name) in names.iter().enumerate() {
		sum += (idx + 1) as u32 * name_score(name);
	}

	assert_eq!(sum, 871198282);
	
	Ok(())
}
