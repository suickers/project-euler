// store word counts as integers rather than doing 
// string conversions.
const ONES: [usize; 10] = [
0, 3, 3, 5, 4, 4, 3, 5, 5, 4];
const TEENS: [usize; 10] = [
3, 6, 6, 8, 8, 7, 7, 9, 8, 8];
const TENS: [usize; 10] = [
0, 0, 6, 6, 5, 5, 5, 7, 6, 6];
    
const HUN: usize = 7;
const AND: usize = 3;

fn parse_number(n: usize) -> usize {
    // extracts the needed digit from an integer
    // for purposes of indexing the above consts 
    let h = (n / 100) % 10;
    let t = (n / 10) % 10;
    let r = n % 10;

    // match case with conditionals. Rust! 
    match n {
        1..=9 => ONES[n],
        10..=19 => TEENS[r],
        20..=99 => TENS[t] + ONES[r],
        100..=999 if t == 0 && r == 0 => ONES[h] + HUN, 
        100..=999 if t == 0 => ONES[h] + HUN + AND + ONES[r],
        100..=999 if t == 1 => ONES[h] + HUN + AND + TEENS[r],
        1000 => 11, 
        _ => ONES[h] + HUN + AND + TENS[t] + ONES[r], 
    }
}

fn main() {
    // adds parse_number for each x in 1..=1000 range
    // to accumulator,a .
    let s = (1..=1000).fold(0, |a, x| a + parse_number(x));
    println!("{}", s); 
}
