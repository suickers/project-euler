use project_euler::primes::*;

fn count_consecutive_primes(a: i32, b: i32) -> i32 {
    let mut n = 0;
    loop {
        let t = (n * n) + (n * a) + b;
        if !is_prime(t) {
            break n;
        }
        n += 1;
    }
}

fn solution() -> i32 {
    let primes: Vec<_> = Primes::new()
        .take_while(|&x| x < 1000)
        .map(|x| x as i32)
        .collect();
        
    let mut max_c = 0;
    let mut max_ab = 0;

    
    for &b in &primes {
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
    assert_eq!(-59231, solution());
}
