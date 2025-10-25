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

	pub struct Primes {
		next: u128,
		done_two: bool,
	}

	impl Primes {
	    pub fn new() -> Self {
	        Self { next: 3, done_two: false }
	    }
	}

	
	impl Iterator for Primes {
	    type Item = u128;
	    
	    fn next(&mut self) -> Option<Self::Item> {
	        if !self.done_two {
	            self.done_two = true;
	            return Some(2);
	        }
	        while self.next < u128::MAX - 2 {
	            let x = self.next;
	            self.next += 2;
	            if is_prime(x) { return Some(x); }
	        }
	        None
	    }
	}
}
