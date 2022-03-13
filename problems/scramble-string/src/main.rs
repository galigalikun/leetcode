fn main() {
    assert_eq!(
        Solution::is_scramble("great".to_string(), "rgeat".to_string()),
        true
    );
    assert_eq!(
        Solution::is_scramble("abcde".to_string(), "caebd".to_string()),
        false
    );
    assert_eq!(
        Solution::is_scramble("a".to_string(), "a".to_string()),
        true
    );
    assert_eq!(
        Solution::is_scramble(
            "eebaacbcbcadaaedceaaacadccd".to_string(),
            "eadcaacabaddaceacbceaabeccd".to_string()
        ),
        false
    );
}

// https://www.geeksforgeeks.org/check-if-a-string-is-a-scrambled-form-of-another-string/
// https://programmerstart.com/article/16641196292/
struct Solution {}
impl Solution {
    pub fn is_scramble(s1: String, s2: String) -> bool {
        let n1 = s1.len();
        let n2 = s2.len();
        if n1 != n2 {
            return false;
        }
        if n1 == 0 {
            return true;
        }
        if s1 == s2 {
            return true;
        }

        let mut dp = vec![vec![vec![false; n1 + 1]; n1]; n1];
        for i in 0..n1 {
            for j in 0..n1 {
                if s1.chars().nth(i) == s2.chars().nth(j) {
                    dp[i][j][1] = true;
                }
            }
        }
        for len in 2..=n1 {
            for i in 0..=n1 - len {
                for j in 0..=n1 - len {
                    for k in 1..=len - 1 {
                        if dp[i][j][k] && dp[i + k][j + k][len - k] {
                            dp[i][j][len] = true;
                            break;
                        }
                        if dp[i][j + len - k][k] && dp[i + k][j][len - k] {
                            dp[i][j][len] = true;
                            break;
                        }
                    }
                }
            }
        }

        return dp[0][0][n1];
    }
}
