use std::time::Instant;
use std::iter::successors; 

fn is_pandigital(x: usize) -> bool {
    let mut arr = [true; 10];
    arr[0] = false;
    
    if x.ilog10() + 1 != 9 { return false; }

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

fn length_of(x: usize) -> u32 {
    x.ilog10() + 1
}

fn main() {
    let time = Instant::now();
    
    let max_p = (1..=9999)
        .flat_map(|i| {
            (1..=9)
                .map(move |n| i*n)
                .scan(0, |state, x| {
                    *state = *state * 10usize.pow(length_of(x)) + x;
                    if length_of(*state) > 9 { return None; }
                    Some(*state)
                })
                .filter(|p| is_pandigital(*p))
        })
        .max()
        .unwrap_or(0);
        
    assert_eq!(max_p, 932718654);
    
    println!("{:?}", time.elapsed()); 
}
