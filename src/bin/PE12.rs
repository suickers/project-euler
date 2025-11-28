use std::time::Instant;

fn tri_num(n: u64) -> (u64, u64) {
    if n % 2 == 0 { (n/2, n+1) }
    else { (n, (n+1)/2) }
}

fn divcount(mut n: u64) -> u64 {
    let mut tot = 1;
    let mut e = 0;
    
    while n % 2 == 0 {
        (n, e) = (n / 2, e + 1);
    } if e > 0 { tot *= e + 1; }
    
    let mut p = 3;
    while p*p <= n {
        e = 0;

        while n % p == 0 { n /= p; e += 1; }
        if e > 0 { tot *= e + 1;}
        p += 2;
    }

    if n > 1 { tot *= 2; }
    
    tot
}

fn main() {
    let time = Instant::now();
    
    let out = (1..)
        .map(|x| tri_num(x))
        .find(|(t1, t2)| divcount(*t1) * divcount(*t2) > 500)
        .map(|(t1, t2)| t1 * t2)
        .unwrap();

    assert_eq!(76576500, out);
    println!("{:?}", time.elapsed());
}
