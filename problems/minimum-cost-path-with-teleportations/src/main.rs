fn main() {
    assert_eq!(
        Solution::min_cost(
            vec![[1, 3, 3], [2, 5, 4], [4, 3, 5]]
                .iter()
                .map(|&x| x.to_vec())
                .collect(),
            2
        ),
        7
    );
    assert_eq!(
        Solution::min_cost(
            vec![[1, 2], [2, 3], [3, 4]]
                .iter()
                .map(|&x| x.to_vec())
                .collect(),
            1
        ),
        9
    );
}

struct Solution;
impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![i32::MAX; n]; m];
        dp[0][0] = 0;
        for i in 0..m {
            for j in 0..n {
                if i > 0 {
                    dp[i][j] = dp[i][j]
                        .min(dp[i - 1][j] + if grid[i - 1][j] != grid[i][j] { k } else { 0 });
                }
                if j > 0 {
                    dp[i][j] = dp[i][j]
                        .min(dp[i][j - 1] + if grid[i][j - 1] != grid[i][j] { k } else { 0 });
                }
            }
        }
        dp[m - 1][n - 1]
    }
}
