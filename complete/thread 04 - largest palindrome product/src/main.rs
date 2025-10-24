fn is_palindrome(n: usize) -> bool {
    let s = n.to_string();
    
    s.chars().eq(s.chars().rev())
}

fn main() {
    let mut max_n = 0; 
    
    for i in 1..1000 {
        for j in 1..1000 {
            let res = if is_palindrome(i*j) { i*j } else { continue };
            if res > max_n { max_n = res; }
        }
    }
    
    assert_eq!(906609, max_n);
}
