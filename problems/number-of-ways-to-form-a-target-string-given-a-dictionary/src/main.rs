fn main() {
    assert_eq!(
        Solution::num_ways(
            vec!["acca".to_string(), "bbbb".to_string(), "caca".to_string()],
            "aba".to_string()
        ),
        6
    );
    assert_eq!(
        Solution::num_ways(
            vec!["abba".to_string(), "baab".to_string()],
            "bab".to_string()
        ),
        4
    );
}

struct Solution;
impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        let mut dp = vec![vec![0; 26]; target.len() + 1];
        let mod_num = 1000000007;
        let target = target.as_bytes();
        let words = words.iter().map(|x| x.as_bytes()).collect::<Vec<_>>();
        for i in 0..26 {
            dp[0][i] = 1;
        }
        for i in 1..=target.len() {
            let mut count = [0; 26];
            for j in 0..words.len() {
                count[words[j][i - 1] as usize - 'a' as usize] += 1;
            }
            for j in 0..26 {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j]) % mod_num;
                dp[i][j] = (dp[i][j] + mod_num - count[j]) % mod_num;
            }
        }
        let mut res = 0;
        for i in 0..26 {
            res = (res + dp[target.len()][i]) % mod_num;
        }
        return res;
    }
}
