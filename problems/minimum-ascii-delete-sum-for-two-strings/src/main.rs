fn main() {
    assert_eq!(
        Solution::minimum_delete_sum("sea".to_string(), "eat".to_string()),
        231
    );
    assert_eq!(
        Solution::minimum_delete_sum("delete".to_string(), "leet".to_string()),
        403
    );
}

struct Solution {}
impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
        for i in 0..s1.len() + 1 {
            for j in 0..s2.len() + 1 {
                if i == 0 {
                    dp[i][j] = j as i32 * s2.chars().nth(j).unwrap_or('\0') as i32;
                } else if j == 0 {
                    dp[i][j] = i as i32 * s1.chars().nth(i).unwrap_or('\0') as i32;
                } else {
                    if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) {
                        dp[i][j] = dp[i - 1][j - 1];
                    } else {
                        dp[i][j] = std::cmp::min(
                            dp[i - 1][j] + s1.chars().nth(i - 1).unwrap() as i32,
                            dp[i][j - 1] + s2.chars().nth(j - 1).unwrap() as i32,
                        );
                    }
                }
            }
        }
        return dp[s1.len()][s2.len()] as i32;
    }
}
