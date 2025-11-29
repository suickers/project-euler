use std::time::Instant;

const IDX_RANGE: [usize; 7] 
	= [9, 180, 2700, 36000, 450000, 5600000, 63000000];

fn digit_at(n: usize) -> usize {
	if n < 10 { return n; }
	
    let chunk = IDX_RANGE.into_iter()
        .scan(n, |state, x| {
            let count = if *state > x { 1 } else { return None; };
            *state -= x;

            Some(count)
        })
        .sum();

    let digit_len = chunk + 1;

    let a = (0..chunk)
        .fold(n, |acc, x| acc - IDX_RANGE[x]);
        
    let q = a / (digit_len);
    let r = a % (digit_len);
    
    let base = 10usize.pow(chunk as u32);

    let num = base + q; 

    if r == 0 {
        (num - 1) % 10
    } else {
        (num / 10usize.pow((digit_len - r) as u32)) % 10
    }

}
fn main() {
	let time = Instant::now();

	let ans: usize = (0..7)
		.map(|x| 10usize.pow(x))
		.map(digit_at)
		.product();

	assert_eq!(ans, 210);
   	println!("{:?}", time.elapsed());
}
