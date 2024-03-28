fn main() {
    assert_eq!(
        Solution::find_good_strings(2, "aa".to_string(), "da".to_string(), "b".to_string()),
        51
    );
    assert_eq!(
        Solution::find_good_strings(
            8,
            "leetcode".to_string(),
            "leetgoes".to_string(),
            "leet".to_string()
        ),
        0
    );
    assert_eq!(
        Solution::find_good_strings(2, "gx".to_string(), "gz".to_string(), "x".to_string()),
        2
    );
}

struct Solution;
impl Solution {
    fn dfs(
        n: usize,
        pos: usize,
        s1: usize,
        s2: usize,
        dp: &mut Vec<Vec<Vec<i32>>>,
        s: &[u8],
        evil: &[u8],
        mod_num: i32,
    ) -> i32 {
        if pos == n {
            return 1;
        }
        if dp[pos][s1][s2] != 0 {
            return dp[pos][s1][s2];
        }
        let mut res = 0;
        let start = if s1 == 1 { s[pos] } else { b'a' };
        let end = if s2 == 1 { s[pos] } else { b'z' };
        for i in start..=end {
            let mut flag = true;
            for j in 0..evil.len() {
                if pos >= evil.len() && i == evil[j] {
                    flag = false;
                    break;
                }
                if pos < evil.len() && i == evil[j] {
                    flag = false;
                    break;
                }
            }
            if !flag {
                continue;
            }
            let s1 = if s1 == 1 && i == s[pos] { 1 } else { 0 };
            let s2 = if s2 == 1 && i == s[pos] { 1 } else { 0 };
            res = (res + Self::dfs(n, pos + 1, s1, s2, dp, s, evil, mod_num)) % mod_num;
        }
        dp[pos][s1][s2] = res;
        return res;
    }
    pub fn find_good_strings(n: i32, s1: String, s2: String, evil: String) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![vec![0; 2]; 2]; n + 1];
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let evil = evil.as_bytes();
        let mod_num = 1000000007;
        dp[0][0][0] = 1;
        return (Self::dfs(n, 0, 1, 1, &mut dp, &s2, &evil, mod_num)
            - Self::dfs(n, 0, 0, 1, &mut dp, &s1, &evil, mod_num)
            + mod_num)
            % mod_num;
    }
}
