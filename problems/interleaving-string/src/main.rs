fn main() {
    assert_eq!(
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string()
        ),
        true
    );

    assert_eq!(
        Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string()
        ),
        false
    );

    assert_eq!(
        Solution::is_interleave("".to_string(), "".to_string(), "".to_string()),
        true
    );
}

struct Solution {}
// https://www.geeksforgeeks.org/find-if-a-string-is-interleaved-of-two-other-strings-dp-33/
impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();

        let mut dp = vec![vec![false; n2 + 1]; n1 + 1];
        if n1 + n2 != s3.len() {
            return false;
        }
        for i in 0..=n1 {
            for j in 0..=n2 {
                if i == 0 && j == 0 {
                    dp[i][j] = true;
                } else if i == 0 {
                    if s2.chars().nth(j - 1) == s3.chars().nth(j - 1) {
                        dp[i][j] = dp[i][j - 1];
                    }
                } else if j == 0 {
                    if s1.chars().nth(i - 1) == s3.chars().nth(i - 1) {
                        dp[i][j] = dp[i - 1][j];
                    }
                } else if s1.chars().nth(i - 1) == s3.chars().nth(i + j - 1)
                    && s2.chars().nth(j - 1) != s3.chars().nth(i + j - 1)
                {
                    dp[i][j] = dp[i - 1][j];
                } else if s1.chars().nth(i - 1) != s3.chars().nth(i + j - 1)
                    && s2.chars().nth(j - 1) == s3.chars().nth(i + j - 1)
                {
                    dp[i][j] = dp[i][j - 1];
                } else if s1.chars().nth(i - 1) == s3.chars().nth(i + j - 1)
                    && s2.chars().nth(j - 1) == s3.chars().nth(i + j - 1)
                {
                    dp[i][j] = dp[i - 1][j] || dp[i][j - 1];
                }
            }
        }

        return dp[n1][n2];
    }
}
