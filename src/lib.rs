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
}
