use std::iter::successors;
use std::time::Instant;

fn is_palindrome(n: usize) -> bool {
    n == reverse(n)
}

fn reverse(n: usize) -> usize {
    successors(Some(n), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
    .fold(0, |acc, d| (acc * 10) + d)
}

fn main() {
    let time = Instant::now();
    
    let max_n = (1..1000)
        .flat_map(|i| (1..1000).map(move |j| i * j))
        .filter(|p| is_palindrome(*p))
        .max()
        .unwrap_or(0);
        
    assert_eq!(906609, max_n);
    println!("{:?}", time.elapsed());
}
