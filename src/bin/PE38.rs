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

fn concat_arr(a: &[usize]) -> usize {
    a.into_iter().try_fold(0, |acc, x| { 
        if acc > 987_654_321 { None } 
        else { Some(acc * 10usize.pow(x.ilog10() + 1) + x) }
    })
    .unwrap_or(1)
}

fn main() {
    let time = Instant::now();
    
    let mut max_p = 0;
    let mut vec = Vec::new(); 
    
    for i in 1..=9999 {
        vec.clear();
        for n in 1..=9 {
            vec.push(i * n); 

            let x = concat_arr(&vec);
            if x.ilog10() + 1 > 9 { break; }
            if is_pandigital(x) && x > max_p { 
                max_p = x; 
            }
        }
    }
    assert_eq!(max_p, 932718654);
    
    println!("{:?}", time.elapsed()); 
}
