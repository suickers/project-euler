use std::cmp::min; 
use std::time::Instant;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn min_cost_sum(grid: &[Vec<usize>]) -> usize {
	let m = grid.len();
	let n = grid[0].len(); 

	let mut dp = vec![vec![0; n]; m];

	dp[0][0] = grid[0][0];

	for i in 1..m {
		dp[i][0] = grid[i][0] + dp[i-1][0];
	}
	for j in 1..n {
		dp[0][j] = grid[0][j] + dp[0][j-1];
	}

	for i in 1..m {
		for j in 1..n {
			dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])
		}
	}

	dp[m-1][n-1]	
}

fn main() -> io::Result<()> {
	let time = Instant::now();

	let f = File::open("data/0081_matrix.txt")?;
	let reader = BufReader::new(f);
	
	let mut grid = Vec::new();

	for line in reader.lines() {
		let split_line = line?
			.split(',')
			.map(|x| x.parse::<usize>().unwrap())
			.collect::<Vec<usize>>();

		grid.push(split_line);
	};

	println!("{}", min_cost_sum(&grid));
	println!("{:?}", time.elapsed());

	Ok(())
}
