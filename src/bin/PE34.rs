use std::time::Instant;
use std::iter::successors;

fn checker(n: usize, facts: &[usize]) -> bool {
    if n <= 2 { return false; }
    
    let sum = successors(Some(n), |x| {
            if *x >= 10 { Some(x / 10) } else { None }
        })
        .map(|x| x % 10)
        .try_fold(0, |acc, x| { 
            if x == 9 { None } else { Some(acc + facts[x]) }
        })
        .unwrap_or(0);
        
    sum == n
}

fn fact(n: usize) -> usize {
    (1..=n).product()
}

fn main() {
    let t = Instant::now();
    
    let limit = 88774;
    
    let facts = (0..10)
        .map(fact)
        .collect::<Vec<usize>>();
    
    let sum = (1..=limit)
        .filter(|x| checker(*x, &facts))
        .sum::<usize>();
    
    assert_eq!(sum, 40730);
    println!("{:?}", t.elapsed());
}
