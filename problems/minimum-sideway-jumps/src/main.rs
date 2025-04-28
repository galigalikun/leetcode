fn main() {
    assert_eq!(Solution::min_side_jumps(vec![0, 1, 2, 3, 0]), 2);
    assert_eq!(Solution::min_side_jumps(vec![0,1,1,3,3,0]), 0);
    assert_eq!(Solution::min_side_jumps(vec![0, 2, 1, 0, 3]), 2);
}

struct Solution;
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let n = obstacles.len();
        let mut dp = vec![vec![i128::MAX; 3]; n];
        dp[0][0] = 0;
        dp[0][1] = 1;
        dp[0][2] = 1;
        for i in 1..n {
            for j in 0..3 {
                if obstacles[i] == j as i32 + 1 {
                    continue;
                }
                dp[i][j] = dp[i - 1][j];
                for k in 0..3 {
                    if k != j && obstacles[i] != k as i32 + 1 {
                        dp[i][j] = dp[i][j].min(dp[i - 1][k] + 1);
                    }
                }
            }
        }
        let mut ans = i32::MAX;
        for j in 0..3 {
            ans = ans.min(dp[n - 1][j] as i32);
        }
        ans
    }
}
