fn solution(a: isize) -> isize {

    // let p = a / 2;
    // let mut m = 0;
    // let mut n = 0;
    
    // for em in 1..p {
    //     for en in 1..em-1 {
    //         if p % (em*(em+en)) == 0 {
    //             (m, n) = (em, en);
    //         }
    //     }
    // }
    
    // (2*m*n)*(m.pow(4)-n.pow(4))
    
    (1..a/2)
        .flat_map(|em| {
            (1..em-1)
                .map(move |en| (em, en))
                .filter(|(em, en)| (a/2) % (em*(em+en)) == 0)
        })
        .next()
        .map(|(m, n)| (2*m*n)*(m.pow(4)-n.pow(4)))
        .unwrap_or(0)
}

fn main() {
    assert_eq!(31875000, solution(1000));
}
