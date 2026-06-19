fn main() {
    assert_eq!(Solution::count_palindromic_subsequences(String::from("bccb")), 6);
    assert_eq!(Solution::count_palindromic_subsequences(String::from("abcdabcdabcdabcdabcdabcdabcdabcddcbadcbadcbadcbadcbadcbadcbadcba")), 104860361);
}

struct Solution{}
impl Solution {
    pub fn count_palindromic_subsequences(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let chars = s.as_bytes();
        let n = chars.len();

        if n == 0 {
            return 0;
        }

        let mut dp = vec![vec![0_i64; n]; n];
        for (i, row) in dp.iter_mut().enumerate() {
            row[i] = 1;
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;

                if chars[i] == chars[j] {
                    let mut left = i + 1;
                    let mut right = j.saturating_sub(1);

                    while left <= right && chars[left] != chars[i] {
                        left += 1;
                    }
                    while left <= right && chars[right] != chars[i] {
                        right = right.saturating_sub(1);
                    }

                    let middle = if i < j.saturating_sub(1) {
                        dp[i + 1][j - 1]
                    } else {
                        0
                    };

                    dp[i][j] = if left > right {
                        (middle * 2 + 2) % MOD
                    } else if left == right {
                        (middle * 2 + 1) % MOD
                    } else {
                        let inner = if left < right.saturating_sub(1) {
                            dp[left + 1][right - 1]
                        } else {
                            0
                        };
                        (middle * 2 - inner) % MOD
                    };
                } else {
                    let val = dp[i + 1][j] + dp[i][j - 1] - dp[i + 1][j - 1];
                    dp[i][j] = val % MOD;
                }

                if dp[i][j] < 0 {
                    dp[i][j] += MOD;
                }
            }
        }

        dp[0][n - 1] as i32
    }
}
