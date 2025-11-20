use std::collections::HashSet;
use std::time::Instant;
use std::iter::successors; 

fn is_pandigital_product(m: usize, a: usize, p: usize) -> bool {
    let mut arr = [true; 10];
    arr[0] = false;

    if (m.ilog10() + 1) + (a.ilog10() + 1) + (p.ilog10() + 1) != 9 {
        return false;
    }

    let x = concat(m, a, p);

    for n in digits_of(x) {
        arr[n] = false;
    }

    arr == [false; 10]
}

fn digits_of(a: usize) -> impl Iterator<Item = usize> {
    successors(Some(a), |x| {
        if *x >= 10 { Some(x / 10) } else { None }
    })
    .map(|x| x % 10)
}

fn concat(a: usize, b: usize, c: usize) -> usize {
    let n1 = b.ilog10() + 1;
    let n2 = c.ilog10() + 1;

    (a * 10usize.pow(n1) + b) * 10usize.pow(n2) + c
}

fn main() {

    let time = Instant::now();
    let mut set = HashSet::new();

    for i in 1..=87 {
        for j in 1..=1965 {
            let product = i*j;
            if is_pandigital_product(i, j, product) {
                set.insert(product);
            }
        }
    }
    let sum: usize = set.into_iter().sum();
    assert_eq!(sum, 45228);

    // 891.074µs on Rust Playground 
    println!("{:?}", time.elapsed()); 
}
