fn solution(a: isize) -> isize {

    let p = a / 2;
    let mut m = 0;
    let mut n = 0;
    
    for em in 1..p {
        for en in 1..em-1 {
            if p % (em*(em+en)) == 0 {
                (m, n) = (em, en);
            }
        }
    }
    
    (2*m*n)*(m.pow(4)-n.pow(4))
}

fn main() {
    assert_eq!(31875000, solution(1000));
}

    
