fn main() {
    assert_eq!(Solution::num_perms_di_sequence("DID".to_string()), 5);
    assert_eq!(Solution::num_perms_di_sequence("D".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn num_perms_di_sequence(s: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = s.len();
        let mut dp = vec![1_i64; n + 1];
        let bytes = s.as_bytes();

        for i in 0..n {
            let mut next = vec![0_i64; n - i];
            if bytes[i] == b'I' {
                let mut prefix = 0_i64;
                for j in 0..(n - i) {
                    prefix = (prefix + dp[j]) % MOD;
                    next[j] = prefix;
                }
            } else {
                let mut suffix = 0_i64;
                for j in (0..(n - i)).rev() {
                    suffix = (suffix + dp[j + 1]) % MOD;
                    next[j] = suffix;
                }
            }
            dp = next;
        }

        dp[0] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_cases() {
        assert_eq!(Solution::num_perms_di_sequence("DID".to_string()), 5);
        assert_eq!(Solution::num_perms_di_sequence("D".to_string()), 1);
    }

    #[test]
    fn additional_cases() {
        assert_eq!(Solution::num_perms_di_sequence("".to_string()), 1);
        assert_eq!(Solution::num_perms_di_sequence("I".to_string()), 1);
        assert_eq!(Solution::num_perms_di_sequence("DI".to_string()), 2);
        assert_eq!(Solution::num_perms_di_sequence("ID".to_string()), 2);
    }
}
