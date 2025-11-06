fn main() {
    let mut d = vec![0usize; 10_000];
    let lim = d.len();
    
    for i in 1..lim { 
        for j in (2*i..lim).step_by(i) { 
            d[j] += i;  
        }
    }
    
    let mut total = 0;
    for a in 2..lim {
        let b = d[a];
        
        if b < lim && d[b] == a && a != b && a < b { 
            total += a + b; 
        }
    }
    
    assert_eq!(total, 31626);
}
