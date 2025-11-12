use std::collections::HashSet;

fn is_pandigital_product(m: usize, a: usize, p: usize) -> bool {
	let mut s = String::new(); 

	let mstr = &m.to_string();
	let astr = &a.to_string();
	let pstr = &p.to_string(); 

	if mstr.len() + astr.len() + pstr.len() != 9 { return false; }
	
	s.push_str(mstr);
	s.push_str(astr);
	s.push_str(pstr);

	let mut v = s.chars().collect::<Vec<char>>();
	if v.contains(&'0') { return false; }
	
	v.sort();

	v == vec!['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

fn main() {
	let mut set = HashSet::new(); 

	for i in 1..=99 {
		for j in 1..=9999 {
			let product = i*j;

			if is_pandigital_product(i, j, product) {
				set.insert(product);
			}
		}
	}

	let sum: usize = set.into_iter().sum();
	
	assert_eq!(sum, 45228);
}
