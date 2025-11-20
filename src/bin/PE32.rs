use std::collections::HashSet;
use std::time::Instant;
use std::iter::successors; 

fn is_pandigital_product(m: usize, a: usize, p: usize) -> bool {
    let mut vec = vec![true; 10];
    vec[0] = false;
    
    if (m.ilog10() + 1) + (a.ilog10() + 1) + (p.ilog10() + 1) != 9 {
        return false;
    }
    
    for n in digits_of(m) {
        vec[n] = false;
    }
    for n in digits_of(a) {
        vec[n] = false;
    }
    for n in digits_of(p) {
        vec[n] = false;
    }
    
    vec == vec![false; 10]
}

fn digits_of(n: usize) -> impl Iterator<Item = usize> {
    successors(Some(n), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
}

fn main() {
    
    let time = Instant::now();
    
    let mut set = HashSet::new(); 
    for i in 1..=99 {
        for j in 1..=2000 {
            let product = i*j;
            if is_pandigital_product(i, j, product) {
                set.insert(product);
            }
        }
    }
    let sum: usize = set.into_iter().sum();

    assert_eq!(sum, 45228);
    
    println!("{:?}", time.elapsed());
}
