use std::time::Instant;

fn primcount(mut n: u64) -> usize {
    let mut count = 0;
    
    if n % 2 == 0 {
    	count += 1;
    	while n % 2 == 0 { n /= 2; }
    }
    
    let mut p = 3;
    while p*p <= n {
		if n % p == 0 {
			count += 1;
			while n % p == 0 { n /= p; }
		}
		p += 2;
    }

    if n > 1 { count += 1; }
    
    count
}

fn main() {
	let time = Instant::now(); 

	let ans = (1..)
		.find(|&n| {
			[n, n+1, n+2, n+3]
				.iter()
				.all(|&x| primcount(x) == 4)
		})
		.unwrap();

	assert_eq!(ans , 134043);
	println!("{:?}", time.elapsed());
}
