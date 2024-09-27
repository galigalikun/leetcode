fn main() {
    assert_eq!(
        Solution::max_product_path(vec![vec![-1, -2, -3], vec![-2, -3, -3], vec![-3, -3, -2]]),
        -1
    );
    assert_eq!(
        Solution::max_product_path(vec![vec![1, -2, 1], vec![1, -2, 1], vec![3, -4, 1]]),
        8
    );
    assert_eq!(Solution::max_product_path(vec![vec![1, 3], vec![0, -4]]), 0);
}

struct Solution;
impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![vec![(0, 0); grid[0].len()]; grid.len()];
        dp[0][0] = (grid[0][0], grid[0][0]);
        for i in 1..grid.len() {
            dp[i][0] = (dp[i - 1][0].0 * grid[i][0], dp[i - 1][0].1 * grid[i][0]);
        }
        for j in 1..grid[0].len() {
            dp[0][j] = (dp[0][j - 1].0 * grid[0][j], dp[0][j - 1].1 * grid[0][j]);
        }
        for i in 1..grid.len() {
            for j in 1..grid[0].len() {
                let mut max = std::i32::MIN;
                let mut min = std::i32::MAX;
                if dp[i - 1][j].0 != 0 {
                    max = max.max(dp[i - 1][j].0 * grid[i][j]);
                    min = min.min(dp[i - 1][j].0 * grid[i][j]);
                }
                if dp[i - 1][j].1 != 0 {
                    max = max.max(dp[i - 1][j].1 * grid[i][j]);
                    min = min.min(dp[i - 1][j].1 * grid[i][j]);
                }
                if dp[i][j - 1].0 != 0 {
                    max = max.max(dp[i][j - 1].0 * grid[i][j]);
                    min = min.min(dp[i][j - 1].0 * grid[i][j]);
                }
                if dp[i][j - 1].1 != 0 {
                    max = max.max(dp[i][j - 1].1 * grid[i][j]);
                    min = min.min(dp[i][j - 1].1 * grid[i][j]);
                }
                dp[i][j] = (max, min);
            }
        }
        let mut res = std::i32::MIN;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                res = res.max(dp[i][j].0);
            }
        }
        if res < 0 {
            return -1;
        }
        res % 1000000007
    }
}
