use std::iter::successors;

fn is_palindrome(n: usize) -> bool {
    let b = format!("{n:b}")
        .parse::<usize>()
        .unwrap();
        
    n == reverse(n) && b == reverse(b)
}

fn reverse(n: usize) -> usize {
    successors(Some(n), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
    .fold(0, |acc, d| (acc * 10) + d)
}

fn main() {
    
    let sum = (1..1_000_000)
        .filter(|x| is_palindrome(*x))
        .sum::<usize>();
    
    assert_eq!(sum, 872187);
}
