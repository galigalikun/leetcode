fn main() {
    assert_eq!(
        Solution::longest_palindrome("cacb".to_string(), "cbba".to_string()),
        5
    );
    assert_eq!(
        Solution::longest_palindrome("ab".to_string(), "ab".to_string()),
        3
    );
    assert_eq!(
        Solution::longest_palindrome("aa".to_string(), "bb".to_string()),
        0
    );
}

struct Solution;
impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let mut res = 0;
        let mut dp = vec![vec![0; 2000]; 2000];
        let s = word1.clone() + &word2;
        let n = s.len();
        for i in (0..n).rev() {
            dp[i][i] = 1;
            for j in i + 1..n {
                if s.as_bytes()[i] == s.as_bytes()[j] {
                    dp[i][j] = dp[i + 1][j - 1] + 2;
                    if i < word1.len() && j >= word1.len() {
                        res = res.max(dp[i][j]);
                    }
                } else {
                    dp[i][j] = dp[i][j - 1].max(dp[i + 1][j]);
                }
            }
        }
        res
    }
}
