fn main() {
    assert_eq!(Solution::number_of_arrays("1000".to_string(), 10000), 1);
    assert_eq!(Solution::number_of_arrays("1000".to_string(), 10), 0);
    assert_eq!(Solution::number_of_arrays("1317".to_string(), 2000), 8);
}

struct Solution;
impl Solution {
    pub fn number_of_arrays(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as i64;
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        for i in 1..=n {
            let mut num = 0;
            for j in (0..i).rev() {
                num = num * 10 + (s[j] - b'0') as i64;
                if num > k {
                    break;
                }
                dp[i] = (dp[i] + dp[j]) % 1000000007;
            }
        }
        return dp[n] as i32;
    }
}
