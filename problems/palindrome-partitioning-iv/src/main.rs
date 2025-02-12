fn main() {
    assert_eq!(Solution::check_partitioning(format!("abcbdd")), true);
    assert_eq!(Solution::check_partitioning(format!("bcbddxy")), false);
}

struct Solution;
impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        for i in 0..n {
            dp[i][i] = true;
        }
        for p in s.windows(2) {
            if p[0] == p[1] {
                dp[0][1] = true;
            }
        }
        for i in 2..n {
            for j in 0..n - i {
                if s[j] == s[j + i] && dp[j + 1][j + i - 1] {
                    dp[j][j + i] = true;
                }
            }
        }
        for i in 1..n - 1 {
            for j in i + 1..n {
                if dp[0][i - 1] && dp[i][j - 1] && dp[j][n - 1] {
                    return true;
                }
            }
        }
        return false;
    }
}
