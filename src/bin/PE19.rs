use std::time::Instant;

const SHIFT: [u8; 12] = [3, 0, 3, 2, 3, 2, 3, 3, 2, 3, 2, 3];

fn is_leap(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

fn month_shift(month: u32, year: u32) -> u8 {
    if month == 2 && is_leap(year) { return 1; }
    SHIFT[(month - 1) as usize]
}

fn main() {
    let time = Instant::now();
    
    // let mut weekday = 1; let mut sunday_firsts = 0;
    
    // for year in 1900..=2000 {
    //     for month in 1..=12 {
    //         if year >= 1901 && weekday == 0 { sunday_firsts += 1; }
    //         weekday = (weekday + month_shift(month, year)) % 7;
    //     }
    // }
    
    
    let sunday_firsts: usize = (1900..=2000)
        .flat_map(|year| { (1..=12).map(move |month| (year, month)) })
            .scan(1, |state, (year, month)| {
                let count = if year >= 1901 && *state == 0 { 1 } else { 0 };
                *state = (*state + month_shift(month, year)) % 7;
                Some(count)
            })
        .sum();
        
    assert_eq!(sunday_firsts, 171);
    println!("{:?}", time.elapsed());
}
