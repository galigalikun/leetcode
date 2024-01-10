fn main() {
    assert_eq!(Solution::min_difficulty(vec![6, 5, 4, 3, 2, 1], 2), 7);
    assert_eq!(Solution::min_difficulty(vec![9, 9, 9], 4), -1);
    assert_eq!(Solution::min_difficulty(vec![1, 1, 1], 3), 3);
}

struct Solution;
impl Solution {
    pub fn min_difficulty(job_difficulty: Vec<i32>, d: i32) -> i32 {
        let n = job_difficulty.len();
        if n < d as usize {
            return -1;
        }
        let mut dp = vec![vec![0; n]; d as usize];
        dp[0][0] = job_difficulty[0];
        for i in 1..n {
            dp[0][i] = job_difficulty[i].max(dp[0][i - 1]);
        }
        for i in 1..d as usize {
            for j in i..n {
                let mut local_max = job_difficulty[j];
                dp[i][j] = i32::MAX;
                for k in (i - 1)..j {
                    local_max = local_max.max(job_difficulty[k + 1]);
                    dp[i][j] = dp[i][j].min(dp[i - 1][k] + local_max);
                }
            }
        }
        return dp[d as usize - 1][n - 1] as i32;
    }
}
