fn main() {
    assert_eq!(Solution::tiling_rectangle(2, 3), 3);
    assert_eq!(Solution::tiling_rectangle(5, 8), 5);
    assert_eq!(Solution::tiling_rectangle(11, 13), 6);
}

struct Solution;
impl Solution {
    pub fn tiling_rectangle(n: i32, m: i32) -> i32 {
        let mut dp = vec![vec![0; m as usize + 1]; n as usize + 1];
        for i in 1..=n as usize {
            for j in 1..=m as usize {
                dp[i][j] = i32::MAX;
                if i == j {
                    dp[i][j] = 1;
                } else {
                    for k in 1..=i / 2 {
                        dp[i][j] = dp[i][j].min(dp[k][j] + dp[i - k][j]);
                    }
                    for k in 1..=j / 2 {
                        dp[i][j] = dp[i][j].min(dp[i][k] + dp[i][j - k]);
                    }
                }
            }
        }
        return dp[n as usize][m as usize];
    }
}
