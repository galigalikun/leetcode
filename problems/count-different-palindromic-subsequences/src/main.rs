fn main() {
    assert_eq!(Solution::count_palindromic_subsequences(String::from("bccb")), 6);
    assert_eq!(Solution::count_palindromic_subsequences(String::from("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba")), 104860361);
}

struct Solution{}
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        let mut dp: Vec<Vec<i128>> = vec![vec![0; s.len()]; s.len()];
        for i in 0..s.len() {
            dp[i][i] = 1;
        }
        for i in 0..s.len() {
            for j in 0..i {
                if s.chars().nth(i) == s.chars().nth(j) {
                    if i - j < 2 {
                        dp[j][i] = 2;
                    } else {
                        dp[j][i] = dp[j + 1][i - 1] * 2 + dp[j][i - 1];
                    }
                } else {
                    dp[j][i] = dp[j][i - 1] + dp[j + 1][i];
                }
            }
        }
        println!("{:?}", dp[0][s.len() - 1]%1000000007);
        return (dp[0][s.len() - 1] % 1000000007) as i32;
    }
}
