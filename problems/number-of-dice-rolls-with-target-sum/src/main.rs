fn main() {
    assert_eq!(Solution::num_rolls_to_target(1, 6, 3), 1);
    assert_eq!(Solution::num_rolls_to_target(2, 6, 7), 6);
    assert_eq!(Solution::num_rolls_to_target(30, 30, 500), 222616187);
}

struct Solution;
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
        let m = 1000000007;
        for i in 1..=n as usize {
            for j in 1..=target as usize {
                if i == 1 && j <= k as usize {
                    dp[i][j] = 1;
                } else {
                    for l in 1..=k as usize {
                        if j >= l {
                            dp[i][j] = (dp[i][j] + dp[i - 1][j - l]) % m;
                        }
                    }
                }
            }
        }
        return dp[n as usize][target as usize];
    }
}
