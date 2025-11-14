use std::iter::successors;
use std::time::Instant;

// note to self add sieve to primes library 
fn sieve(n: usize) -> Vec<bool> {
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

fn digits_of(n: usize) -> Vec<usize> {
    successors(Some(n), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
    .collect::<Vec<usize>>()
}

fn rotations(n: usize) -> impl Iterator<Item = usize> {
    let digits = digits_of(n);
    let length = digits.len();
    
    (0..length).map(move |shift| {
        (0..length)
            .map(|i| digits[(i + shift) % length])
            .fold(0, |acc, d| acc * 10 + d)
    })
}

fn is_circular(n: usize, sieve: &[bool]) -> bool {
    rotations(n).all(|x| sieve[x])
}

fn main() {
    let time = Instant::now();
    let x = 1_000_000;
    let sieve = sieve(x);
    
    let c = (1..x)
        .filter(|&n| is_circular(n, &sieve))
        .count();
    
    assert_eq!(c, 55);
    println!("{:?}", time.elapsed());
}
