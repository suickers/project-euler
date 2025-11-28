use std::time::Instant;

fn check_triangle(a: usize, b: usize, p: usize) -> bool {
    let c = p - b - a;
    
    (a * a) + (b * b) == c * c
}

fn main() {
    let time = Instant::now();
    
    let max_p = (1..=1000)
        .map(|p| {
            let count = (1..=p/3)
                .flat_map(|a| (1..=p/2).map(move |b| (a, b)))
                .filter(|(a, b)| check_triangle(*a, *b, p))
                .count();
                
            (p, count)
        })
        .max_by_key(|(_, count)| *count)
        .map(|(p, _)| p)
        .unwrap();
        
    assert_eq!(max_p, 840);
    println!("{:?}", time.elapsed());
}
