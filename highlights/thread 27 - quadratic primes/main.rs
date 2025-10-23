use std::time::Instant;

// a "done two" flag to allow for self.next += 2 
// in the iterator. 
struct Primes {
    next: i32,
    done_two: bool,
}

impl Primes {
    fn new() -> Self {
        Self { next: 3, done_two: false }
    }
}

impl Iterator for Primes {
    type Item = i32;
    
    fn next(&mut self) -> Option<Self::Item> {
        // changes self done to true and returns 
        // 2 on the first pass
        if !self.done_two {
            self.done_two = true;
            return Some(2);
        }
        // assign self.next to x before changing it
        // check x is prime with helper function
        while self.next < i32::MAX - 2 {
            let x = self.next;
            self.next += 2;
            if is_prime(x) { return Some(x); }
        }
        None
    }
}

// standard is_prime helper 
fn is_prime(n: i32) -> bool {
    if n < 2 { return false }
    if matches!(n, 2 | 3 | 5) { return true; }
    if n % 2 == 0 || n % 3 == 0 || n % 5 == 0 { return false; }
    
    let mut c: i64 = 7;
    while c * c <= n as i64 {
        if n as i64 % c == 0 
            || n as i64 % (c + 4) == 0 { return false; }
        c += 6;
    }
    true 
}

// counts consecutive primes in the quadratic formula 
// n^2 + an + b , breaking and returning the count, n,
// when a not prime is found 
fn count_consecutive_primes(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        // quadratic formula here
        let t = (n * n) + (n * a) + b;
        if !is_prime(t) {
            break n;
        }
        n += 1;
    }
}

fn solution() -> i32 {
    // populates vector with primes, 
    // up to 1000 as is needed for problem 
    let primes: Vec<_> = Primes::new()
        .take_while(|&x| x < 1000)
        .collect();
        
    let mut max_c = 0;
    let mut max_ab = 0;

    
    for &b in &primes {
        // selecting even As if b == 2 and odd As if
        // b != 2. note step_by(2) on the range
        let start = if b == 2 { -998 } else { -999 };
        for a in (start..=999).step_by(2) {
            let c = count_consecutive_primes(a, b);
            if c > max_c {
                max_c = c;
                max_ab = a * b;
            }
        }
    }
    max_ab
}

fn main() {
    let t = Instant::now();
    println!("{}, {:?}", solution(), t.elapsed()); // -59231, 8.188871ms
}
