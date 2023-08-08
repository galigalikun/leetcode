fn main() {
    assert_eq!(
        Solution::longest_common_subsequence("abcde".to_string(), "ace".to_string()),
        3
    );
    assert_eq!(
        Solution::longest_common_subsequence("abc".to_string(), "abc".to_string()),
        3
    );
}

struct Solution;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut dp = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        let text1 = text1.as_bytes();
        let text2 = text2.as_bytes();
        for i in 1..=text1.len() {
            for j in 1..=text2.len() {
                if text1[i - 1] == text2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                    continue;
                }
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
        return dp[text1.len()][text2.len()] as i32;
    }
}
