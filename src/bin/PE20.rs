use num_bigint::BigUint;
use num_traits::One;

fn main() {
    let fact_sum = |n: usize| -> u32 { (1..=n)
        .map(BigUint::from)
        .fold(BigUint::one(), |acc, x| acc * x) 
        .to_radix_be(10)
        .iter()
        .map(|&d| d as u32)
        .sum() };
        
    assert_eq!(fact_sum(100), 648);
}
