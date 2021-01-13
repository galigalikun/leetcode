fn main() {
    assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
    assert_eq!(Solution::is_match("aa".to_string(), "a*".to_string()), true);
    assert_eq!(Solution::is_match("ab".to_string(), ".*".to_string()), true);
    assert_eq!(Solution::is_match("aab".to_string(), "c*a*b".to_string()), true);
    assert_eq!(Solution::is_match("mississippi".to_string(), "mis*is*p*.".to_string()), false);
    assert_eq!(Solution::is_match("ab".to_string(), ".*c".to_string()), false);
    assert_eq!(Solution::is_match("aaa".to_string(), "a*a".to_string()), true);
    assert_eq!(Solution::is_match("aaa".to_string(), "ab*a".to_string()), false);
    assert_eq!(Solution::is_match("a".to_string(), "ab*".to_string()), true);
    assert_eq!(
        Solution::is_match("bbbba".to_string(), ".*a*a".to_string()),
        true
    );
    assert_eq!(Solution::is_match("ab".to_string(), ".*..".to_string()), true);
    assert_eq!(Solution::is_match("a".to_string(), ".*..a*".to_string()), false);
}

pub struct Solution {}
// https://www.youtube.com/watch?v=Mu_TvHcg5eI
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut dp: Vec<Vec<bool>> = Vec::new();
        for i in 0..=s.as_str().chars().count() {
            dp.push(vec![]);
            for _j in 0..=p.as_str().chars().count() {
                dp[i].push(false);
            }
        }
        dp[0][0] = true;
        let mut ss = Vec::new();
        let mut pattern = Vec::new();
        ss.push('\0');
        pattern.push('\0');
        for i in 0..s.as_str().chars().count() {
            ss.push(s.as_str().chars().nth(i).unwrap());
        }
        for i in 0..p.as_str().chars().count() {
            pattern.push(p.as_str().chars().nth(i).unwrap());
        }

        for i in 1..pattern.len() {
            if pattern[i] == '*' && dp[0][i - 2] {
                dp[0][i] = true;
            }
        }

        for i in 1..ss.len() {
            for j in 1..pattern.len() {
                if pattern[j] == ss[i] || pattern[j] == '.' {
                    dp[i][j] = dp[i - 1][j - 1];
                } else if pattern[j] == '*' {
                    if pattern[j - 1] == ss[i] || pattern[j-1] == '.'{
                        dp[i][j] = dp[i][j - 2] || dp[i - 1][j];
                    } else if j > 1 {
                        dp[i][j] = dp[i][j - 2];
                    }
                }
            }
        }

        return dp[s.as_str().chars().count()][p.as_str().chars().count()];
    }
}
