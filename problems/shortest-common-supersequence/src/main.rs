fn main() {
    assert_eq!(
        Solution::shortest_common_supersequence("abac".to_string(), "cab".to_string()),
        "cabac"
    );
    assert_eq!(
        Solution::shortest_common_supersequence("aaaaaaaa".to_string(), "aaaaaaaa".to_string()),
        "aaaaaaaa"
    );
}

struct Solution;
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let mut dp = vec![vec![0; str2.len() + 1]; str1.len() + 1];
        let mut result = String::new();
        let mut i = str1.len();
        let mut j = str2.len();
        dp[0][0] = 0;
        for i in 1..=str1.len() {
            dp[i][0] = i;
        }
        for j in 1..=str2.len() {
            dp[0][j] = j;
        }
        for i in 1..=str1.len() {
            for j in 1..=str2.len() {
                if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::min(dp[i - 1][j], dp[i][j - 1]) + 1;
                }
            }
        }
        while i > 0 && j > 0 {
            if str1.chars().nth(i - 1) == str2.chars().nth(j - 1) {
                result.push(str1.chars().nth(i - 1).unwrap());
                i -= 1;
                j -= 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                result.push(str1.chars().nth(i - 1).unwrap());
                i -= 1;
            } else {
                result.push(str2.chars().nth(j - 1).unwrap());
                j -= 1;
            }
        }
        while i > 0 {
            result.push(str1.chars().nth(i - 1).unwrap());
            i -= 1;
        }
        while j > 0 {
            result.push(str2.chars().nth(j - 1).unwrap());
            j -= 1;
        }
        return result.chars().rev().collect();
    }
}
