fn main() {
    assert_eq!(
        Solution::min_falling_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        13
    );
    assert_eq!(Solution::min_falling_path_sum(vec![vec![7]]), 7);
}

struct Solution;
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid[0].len() {
            dp[0][i] = grid[0][i];
        }
        for i in 1..grid.len() {
            for j in 0..grid[0].len() {
                dp[i][j] = grid[i][j]
                    + std::cmp::min(
                        dp[i - 1][j],
                        std::cmp::min(
                            dp[i - 1][std::cmp::max(j as i32 - 1, 0) as usize],
                            dp[i - 1]
                                [std::cmp::min(j as i32 + 1, grid[0].len() as i32 - 1) as usize],
                        ),
                    );
            }
        }
        let mut min = dp[grid.len() - 1][0];
        for i in 1..grid[0].len() {
            if dp[grid.len() - 1][i] < min {
                min = dp[grid.len() - 1][i];
            }
        }
        return min;
    }
}
