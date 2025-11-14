pub mod primes {
	use num_traits::{PrimInt, ToPrimitive};
	
	pub fn is_prime<T>(n: T) -> bool 
	where
		T: PrimInt + ToPrimitive
	{
		let Some(n) = n.to_u128() else { return false };
				
		if n < 2 { return false }
		if matches!(n, 2 | 3 | 5) { return true; }
		if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 { return false; }
		
		let mut c: u128 = 7;
		while c <= n / c {
		    if n % c == 0 || n % (c + 4) == 0 { return false; }
		    c += 6;
		}
		true 
	}

	pub fn sieve<T>(n: T) -> Vec<bool>
	where
		T: PrimInt + ToPrimitive
	{
		let n = n.to_usize().unwrap();

		let mut vec = vec![true; n + 1];
	    vec[0] = false; 
	    vec[1] = false;
	    
	    for i in (4..=n).step_by(2) {
	        vec[i] = false;
	    }
	    
	    let mut p = 3;
	    
	    while p <= n / p {
	        if vec[p] {
	            for i in (p*p..=n).step_by(p) {
	                vec[i] = false;
	            }
	        }
	        p += 2;
	    }
	    vec
	}

	pub struct Primes {
		next: u128,
		done_two: bool,
	}

	impl Primes {
	    pub fn new() -> Self {
	        Self { next: 3, done_two: false }
	    }
	}

	impl Default for Primes {
		fn default() -> Self {
			Self::new()
		}
	}
	
	impl Iterator for Primes {
	    type Item = u128;
	    
	    fn next(&mut self) -> Option<Self::Item> {
	        if !self.done_two {
	            self.done_two = true;
	            return Some(2);
	        }
	        
          	loop {
          		let x = self.next;

          		if let Some(v) = self.next.checked_add(2) {
          			self.next = v;
          		}
          		
          		if is_prime(x) {
          			break Some(x);
          		} else {
          			continue;
          		}
          	}	
		}
	}
}

pub mod kibonacci {
	use std::collections::VecDeque;
	use num::bigint::BigUint; 
	use num_traits::{Zero, One};
	
	pub struct KBonacci {
		window: VecDeque<BigUint>,
	}
	
	impl KBonacci {
		pub fn new(k: usize) -> Self {
			assert!(k >= 2, "K must be greater than 1.");
			
			let mut vec = VecDeque::new();
			for _ in 0..k-1 {
				vec.push_front(BigUint::zero());
			}
			vec.push_back(BigUint::one());
	
			Self { window: vec }
		}
	}
	
	impl Iterator for KBonacci {
		type Item = BigUint; 
	
		fn next(&mut self) -> Option<Self::Item> {
			let sum = self.window.iter().sum();
			let x = self.window.pop_front().unwrap();
			self.window.push_back(sum);
	
			Some(x)
		}
	}
}
