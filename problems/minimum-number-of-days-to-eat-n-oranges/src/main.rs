fn main() {
    assert_eq!(Solution::min_days(10), 4);
    assert_eq!(Solution::min_days(6), 3);
}

struct Solution;
impl Solution {
    pub fn min_days(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;
        dp[1] = 1;
        for i in 2..=n as usize {
            dp[i] = dp[i - 1] + 1;
            if i % 2 == 0 {
                dp[i] = dp[i].min(dp[i / 2] + 1);
            }
            if i % 3 == 0 {
                dp[i] = dp[i].min(dp[i / 3] + 1);
            }
        }
        dp[n as usize]
    }
}
