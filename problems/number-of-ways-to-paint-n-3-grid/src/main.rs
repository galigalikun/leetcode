fn main() {
    assert_eq!(Solution::num_of_ways(1), 12);
    assert_eq!(Solution::num_of_ways(5000), 30228214);
}

struct Solution;
impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut dp = vec![vec![0; 3]; n as usize];
        dp[0][0] = 6;
        dp[0][1] = 6;
        dp[0][2] = 6;
        let m = 1_000_000_007;
        for i in 1..n as usize {
            dp[i][0] = (dp[i - 1][0] * 3 + dp[i - 1][1] * 2 + dp[i - 1][2] * 2) % m;
            dp[i][1] = (dp[i - 1][0] * 2 + dp[i - 1][1] * 2 + dp[i - 1][2] * 3) % m;
            dp[i][2] = (dp[i - 1][0] * 2 + dp[i - 1][1] * 3 + dp[i - 1][2] * 2) % m;
        }
        return (dp[n as usize - 1][0] + dp[n as usize - 1][1] + dp[n as usize - 1][2]) as i32;
    }
}
