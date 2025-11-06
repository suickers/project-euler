
// using dynamic programming 
fn count_paths(grid: &[Vec<usize>]) -> usize {
    let m = grid.len();
    let n = grid[0].len();
    
    let mut dp = vec![1; n];
    
    for _ in 1..m {
        for j in 1..n {
            dp[j] += dp[j - 1];
        }
    }
    dp[n - 1]
}

fn main() {
	// using combinatorics 
    let lattice_paths = |n: u64| (1..=n).fold(1, |c, k| (c *(n + k))/k);
	assert_eq!(lattice_paths(20), 137846528820);

	let n = 20;
	let grid = vec![vec![0; n + 1]; n + 1];
    assert_eq!(count_paths(&grid), 137846528820);
}
