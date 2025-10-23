fn sieve(n: usize) -> Vec<usize> {
    let mut vec = vec![true; n + 1];
    vec[0] = false;
    vec[1] = false; 
    
    vec[2] = true;
    for i in (4..=n).step_by(2) {
        vec[i] = false;
    }
    
    let mut p = 3; 
    while p <= n / p {
        if vec[p] == true {
            for i in (p*p..=n).step_by(p) {
                vec[i] = false;
            }
        }
        p += 2;
    }
    
    let mut res = Vec::new(); 
    for (idx, &b) in vec.iter().enumerate() {
        if b { res.push(idx) }
    }
    
    res
}

fn main() {
    let ans: usize = sieve(2_000_000).into_iter().sum();
    assert_eq!(142913828922, ans);
}
