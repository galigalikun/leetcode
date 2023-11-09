fn main() {
    assert_eq!(Solution::palindrome_partition("abc".to_string(), 2), 1);
    assert_eq!(Solution::palindrome_partition("aabbc".to_string(), 3), 0);
    assert_eq!(Solution::palindrome_partition("leetcode".to_string(), 8), 0);
}

struct Solution;
impl Solution {
    pub fn palindrome_partition(s: String, k: i32) -> i32 {
        let mut dp = vec![vec![0; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i] = 0;
        }
        for i in 0..s.len() - 1 {
            dp[i][i + 1] = if s.chars().nth(i) == s.chars().nth(i + 1) {
                0
            } else {
                1
            };
        }
        for i in (0..s.len() - 2).rev() {
            for j in i + 2..s.len() {
                dp[i][j] = if s.chars().nth(i) == s.chars().nth(j) {
                    dp[i + 1][j - 1]
                } else {
                    dp[i + 1][j - 1] + 1
                };
            }
        }
        let mut dp2 = vec![vec![0; s.len()]; k as usize];
        for i in 0..s.len() {
            dp2[0][i] = dp[0][i];
        }
        for i in 1..k as usize {
            for j in i..s.len() {
                dp2[i][j] = std::i32::MAX;
                for m in i - 1..j {
                    dp2[i][j] = std::cmp::min(dp2[i][j], dp2[i - 1][m] + dp[m + 1][j]);
                }
            }
        }
        if k as usize <= s.len() {
            return dp2[k as usize - 1][s.len() - 1];
        }
        return 0;
    }
}
