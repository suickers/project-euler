
const COINS: &[usize] = &[1, 2, 5,10,20,50,100,200];
const SUM: usize = 200;

fn count(coins: &[usize], sum: usize) -> usize {
    let mut dp = vec![0; sum + 1];
    dp[0] = 1;
    
    for &coin in coins {
        for j in coin..=sum {
            dp[j] += dp[j - coin];
        }
    }
    
    dp[sum]
}

fn main() {
    assert_eq!(count(COINS, SUM), 73682);
}
