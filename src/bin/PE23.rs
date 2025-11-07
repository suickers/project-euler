// it's kind of slow. might come back to it to optimise

use project_euler::primes::Primes; 

fn sum_divisors(num: usize, primes: &[usize]) -> usize {
	let mut n = num;
	let mut sum = 1; 

	for &p in primes {
		if p <= n / p {
			let mut temp = 0;
			let mut e = 0;

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
		} else { break; }
	}
	if n > 1 { sum *= 1 + n; }
	sum - num
}

fn is_abundant(n: usize, primes: &[usize]) -> bool {
	if n < 12 { return false; }
	if sum_divisors(n, primes) > n { return true; }
	false
}

fn main() {
	let primes: Vec<_> = Primes::new()
		.take_while(|&x| x < 168)
		.map(|x| x as usize)
		.collect();
		
	let mut abundants = Vec::new(); 
	for i in 1..=28123 {
		if is_abundant(i, &primes) {
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
	assert_eq!(sum, 4179871);
}
