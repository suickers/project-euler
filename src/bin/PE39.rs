use std::time::Instant;

fn check_triangle(a: usize, b: usize, p: usize) -> bool {
    let c = p - b - a;
    
    (a * a) + (b * b) == c * c
}

fn main() {
    let time = Instant::now();
    
    let mut max_p = 0;
    let mut max_count = 0;
    
    for p in 1..=1000 {
        
        let mut count = 0;
    
        for a in 1..=p/3 {
            for b in a..=p/2 {
                
                if check_triangle(a, b, p) {
                    count += 1; 
                }
                
            }
        }
        
        if count > max_count {
            max_count = count;
            max_p = p;
        }
    }
    
    
    println!("{}", max_p);
    println!("{:?}", time.elapsed());
}
