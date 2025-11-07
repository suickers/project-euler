// it's kind of slow. might come back to it to optimise

fn sum_divisors(num: usize) -> usize {
	let mut n = num;
	let mut e = 0;
	let mut sum = 1; 
	let mut temp = 0;
	
	while n % 2 == 0 {
		n /= 2; 
		e += 1; 
	}
	if e > 0 {
		for i in 1..=e {
			temp += 2usize.pow(i);
		}
	}
	sum *= 1 + temp;
	
	
	let mut p = 3;
	while p <= n / p {
		temp = 0;
		e = 0;
		while n % p == 0 { 
			n /= p; 
			e += 1; 
		}
		if e > 0 {
			for i in 1..=e {
				temp += p.pow(i);
			}
		}
		sum *= 1 + temp;
		p += 2; 
	}

	if n > 1 { sum *= 1 + n; }
	sum - num
}

fn is_abundant(n: usize) -> bool {
	if n < 12 { return false; }
	if sum_divisors(n) > n { return true; }
	false
}


fn main() {
	let mut abundants = Vec::new(); 
	for i in 1..=28123 {
		if is_abundant(i) {
			abundants.push(i);
		}
	}
	
	let lim = abundants.len();
	let mut abundant_sums = vec![false; 28123 + 1];
	
	for i in 0..lim {
		for j in 0..lim {
			if abundants[i] + abundants[j] <= 28123 {
				abundant_sums[abundants[i] + abundants[j]] = true;
			} else {
				break;
			}
			
		}
	}

	let mut sum = 0; 

	for (idx, &is_sum) in abundant_sums.iter().enumerate() {
		if idx > 0 && !is_sum {
			sum += idx;
		}
	}
	println!("{}", sum);
}
