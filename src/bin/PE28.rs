fn solution(mut n: i32) -> i32 {
    n = (n - 1) / 2;
    2 * n * ( (8 * n * n) + (15 * n) + 13) / 3 + 1
}

fn main() {
    assert_eq!(solution(1001), 669171001);
}
