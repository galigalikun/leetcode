fn main() {
    assert_eq!(Solution::number_of_combinations("327".to_string()), 2);
    assert_eq!(Solution::number_of_combinations("094".to_string()), 0);
    assert_eq!(Solution::number_of_combinations("0".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn number_of_combinations(num: String) -> i32 {
        let mut dp = vec![0; num.len() + 1];
        dp[0] = 1;
        let num = num.as_bytes();
        let modulo = 1_000_000_007;
        for i in 1..=num.len() {
            if num[i - 1] == b'0' {
                continue;
            }
            for j in (0..i).rev() {
                if num[j] == b'0' {
                    continue;
                }
                let len = i - j;
                if j >= len && &num[j..i] < &num[j - len..j] {
                    dp[i] = (dp[i] + dp[j]) % modulo;
                } else if j > len {
                    dp[i] = (dp[i] + dp[j]) % modulo;
                }
            }
        }
        dp[num.len()] as i32
    }
}
