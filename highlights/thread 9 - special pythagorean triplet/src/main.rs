// euclids formula instead of straight brute force
// a=m^2−n^2, b=2mn, c=m^2+n^2 for all positive integers m > n.
// 
// substituting into a+b+c gives (m^2−n^2)+2mn+(m^2+n^2)
// or, 2m(m+n) 
//
// so if a + b + c == 1000; 2m(m+n) == 1000;
// or, m(m+n) == 500; 
// 
// signed integer to handle the -n^4 later on 
fn solution(a: isize) -> isize {

    let p = a / 2;
    let mut m = 0;
    let mut n = 0;
  
    // look for m, n such that m > n and m(m+n) | (p/2)    
    for em in 1..p {
        for en in 1..em-1 {
            if p % (em*(em+en)) == 0 {
                (m, n) = (em, en);
            }
        }
    }
    // pe #9 wants the product abc, so a bit of rearranging of 
    // m^2−n^2 * 2mn * m^2+n^2 gives 2mn(m^4 - n^4) 
    (2*m*n)*(m.pow(4)-n.pow(4))
}

fn main() {
    assert_eq!(31875000, solution(1000));
}

    
