fn main() {
    assert_eq!(Solution::num_decodings("*".to_string()), 9);
    assert_eq!(Solution::num_decodings("1*".to_string()), 18);
    assert_eq!(Solution::num_decodings("2*".to_string()), 15);
    assert_eq!(Solution::num_decodings("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*".to_string()), 196465252);
}

// https://github.com/cherryljr/LeetCode/blob/master/Decode%20Ways%20II.java
struct Solution {}
impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let r#mod = 1000000007;
        let mut dp = vec![0_i64; s.len() + 1];
        dp[0] = 1;
        dp[1] = if s.chars().nth(0) == Some('*') {
            9
        } else if s.chars().nth(0) == Some('0') {
            0
        } else {
            1
        };

        for i in 2..=s.len() {
            if s.chars().nth(i - 1) == Some('*') {
                dp[i] = 9 * dp[i - 1];
                if s.chars().nth(i - 2) == Some('1') {
                    dp[i] = (dp[i] + 9 * dp[i - 2]) % r#mod;
                } else if s.chars().nth(i - 2) == Some('2') {
                    dp[i] = (dp[i] + 6 * dp[i - 2]) % r#mod;
                } else if s.chars().nth(i - 2) == Some('*') {
                    dp[i] = (dp[i] + 15 * dp[i - 2]) % r#mod;
                }
            } else {
                dp[i] = if s.chars().nth(i - 1) == Some('0') {
                    0
                } else {
                    dp[i - 1]
                };
                if s.chars().nth(i - 2) == Some('1') {
                    dp[i] = (dp[i] + dp[i - 2]) % r#mod;
                } else if s.chars().nth(i - 2) == Some('2') && s.chars().nth(i - 1) <= Some('6') {
                    dp[i] = (dp[i] + dp[i - 2]) % r#mod;
                } else if s.chars().nth(i - 2) == Some('*') {
                    dp[i] = (dp[i]
                        + if s.chars().nth(i - 1) <= Some('6') {
                            2
                        } else {
                            1
                        } * dp[i - 2])
                        % r#mod;
                }
            }
        }

        return dp[s.len()] as i32;
    }
}
