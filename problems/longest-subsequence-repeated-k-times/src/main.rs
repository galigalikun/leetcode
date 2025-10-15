fn main() {
    assert_eq!(Solution::longest_subsequence_repeated_k("letsleetcode".to_string(), 2), "let".to_string());
    assert_eq!(Solution::longest_subsequence_repeated_k("bb".to_string(), 2), "b".to_string());
    assert_eq!(Solution::longest_subsequence_repeated_k("ab".to_string(), 2), "".to_string());
}

struct Solution;
impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let n = s.len();
        let s = s.as_bytes();
        let mut freq = [0; 26];
        for &c in s.iter() {
            freq[(c - b'a') as usize] += 1;
        }
        let mut chars = vec![];
        for i in 0..26 {
            if freq[i] >= k {
                chars.push((i as u8 + b'a') as char);
            }
        }
        let m = chars.len();
        let mut ans = vec![];
        for mask in (1..(1 << m)).rev() {
            let mut candidate = vec![];
            for i in 0..m {
                if (mask & (1 << i)) != 0 {
                    candidate.push(chars[i]);
                }
            }
            let mut dp = vec![0; n + 1];
            for &c in candidate.iter() {
                let mut new_dp = dp.clone();
                for i in 0..n {
                    if s[i] as char == c {
                        new_dp[i + 1] = dp[i] + 1;
                    } else {
                        new_dp[i + 1] = new_dp[i].max(dp[i + 1]);
                    }
                }
                dp = new_dp;
            }
            if dp[n] >= k * candidate.len() as i32 {
                ans = candidate;
                break;
            }
        }
        if !ans.is_empty() {
            return ans.into_iter().collect();
        }
        return String::new();
    }
}
