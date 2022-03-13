pub fn main() {
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
        7
    );
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        12
    );
    assert_eq!(Solution::min_path_sum(vec![vec![1, 2], vec![1, 1]]), 3);
    assert_eq!(
        Solution::min_path_sum(vec![vec![1, 2, 5], vec![3, 2, 1]]),
        6
    );
}

struct Solution {}
// https://www.programcreek.com/2014/05/leetcode-minimum-path-sum-java/
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        /*
        1,2,5
        3,2,1
        */
        if grid.len() == 0 {
            return 0;
        }

        let m = grid.len();
        let n = grid[0].len();

        let mut dp = Vec::with_capacity(m);

        for i in 0..m {
            dp.push(vec![0]);
            for _j in 1..n {
                dp[i].push(0);
            }
        }

        dp[0][0] = grid[0][0];

        for i in 1..n {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }

        for j in 1..m {
            dp[j][0] = dp[j - 1][0] + grid[j][0];
        }

        for i in 1..m {
            for j in 1..n {
                if dp[i - 1][j] > dp[i][j - 1] {
                    dp[i][j] = dp[i][j - 1] + grid[i][j];
                } else {
                    dp[i][j] = dp[i - 1][j] + grid[i][j];
                }
            }
        }

        return dp[m - 1][n - 1];
    }
}
