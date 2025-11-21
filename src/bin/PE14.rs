use std::time::Instant;

fn length(n: usize, memo: &mut Vec<Option<usize>>) -> usize {
	
	let memlen = memo.len();
	
	if n == 1 {
	    return 0;
	}
	
	if n < memlen {
	    if let Some(v) = memo[n] {
    		return v; 
    	}
	}
	
	let next = if n % 2 == 0 { n / 2 } else { (3*n) + 1 };
	
	let v = 1 + length(next, memo);
	
	if n < memlen {
	    memo[n] = Some(v);
	}
	
	v
}

fn main() {
    let time = Instant::now();
    let lim = 1_000_000;
    
	let mut max_n = 0;
	let mut max_s = 0;
	let mut memo = vec![None; lim + 1];
	
	for i in 1..lim {
		let s = length(i, &mut memo);

		if s > max_s {
			max_s = s;
			max_n = i;
		}
	}
	assert_eq!(max_n, 837799);
	
	
	
	println!("{:?}", time.elapsed());
	// ~ 384.100087ms with hashmap and no insert check
	// ~ 248.277903ms with hashmap and n < 1mil check
	// ~ 29.500366ms with vector memo 
}
