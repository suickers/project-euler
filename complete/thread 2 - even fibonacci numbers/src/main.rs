use std::iter::successors;

fn main() {
    // more explicit 
    let fib_sum = |limit: usize| -> usize {
        let mut sum = 0;
        let mut prev = 0;
        let mut curr = 1; 
        
        while curr < limit {
            if curr % 2 == 0 { sum += curr; } 
            (prev, curr) = (curr, prev + curr);
        }
        sum
    };
    
    // more concise. iter methods
    let fib_sum2: usize = successors(Some((0, 1)), |&(a, b)| Some((b, a + b)))
        .map(|x| x.1)
        .take_while(|&x| x < 4_000_000)
        .filter(|x| x % 2 == 0)
        .sum();
    
    assert_eq!(4613732, fib_sum(4_000_000));
    assert_eq!(4613732, fib_sum2);
}
