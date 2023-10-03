fn main() {
    assert_eq!(Solution::count_vowel_permutation(1), 5);
    assert_eq!(Solution::count_vowel_permutation(2), 10);
    assert_eq!(Solution::count_vowel_permutation(5), 68);
    assert_eq!(Solution::count_vowel_permutation(144), 18208803);
}

struct Solution;
impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        let mut dp: Vec<Vec<i128>> = vec![vec![0; 5]; n as usize];
        for i in 0..5 {
            dp[0][i] = 1;
        }
        for i in 1..dp.len() {
            dp[i][0] = (dp[i][0] + dp[i - 1][1]) % 1000000007;
        }
        for i in 1..dp.len() {
            dp[i][1] = (dp[i][1] + dp[i - 1][0]) % 1000000007;
            dp[i][1] = (dp[i][1] + dp[i - 1][2]) % 1000000007;
        }
        for i in 1..dp.len() {
            dp[i][2] = (dp[i][2] + dp[i - 1][0]) % 1000000007;
            dp[i][2] = (dp[i][2] + dp[i - 1][1]) % 1000000007;
            dp[i][2] = (dp[i][2] + dp[i - 1][3]) % 1000000007;
            dp[i][2] = (dp[i][2] + dp[i - 1][4]) % 1000000007;
        }
        for i in 1..dp.len() {
            dp[i][3] = (dp[i][3] + dp[i - 1][2]) % 1000000007;
            dp[i][3] = (dp[i][3] + dp[i - 1][4]) % 1000000007;
        }
        for i in 1..dp.len() {
            dp[i][4] = (dp[i][4] + dp[i - 1][0]) % 1000000007;
        }
        let mut ans = 0;
        for i in 0..5 {
            ans = (ans + dp[n as usize - 1][i]) % 1000000007;
        }
        return ans as i32;
    }
}
