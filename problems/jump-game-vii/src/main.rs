fn main() {
    assert_eq!(Solution::can_reach("011010".to_string(), 2, 3), true);
    assert_eq!(Solution::can_reach("01101110".to_string(), 2, 3), false);
}
struct Solution;
impl Solution {
    fn can_reach_impl(s: &[u8], min_jump: usize, max_jump: usize) -> bool {
        let n = s.len();
        if s[n - 1] != b'0' {
            return false;
        }
        let mut dp = vec![false; n];
        dp[0] = true;
        let mut max_reachable = 0;
        for i in 0..n {
            if !dp[i] {
                continue;
            }
            if i + min_jump < n {
                max_reachable = max_reachable.max(i + min_jump);
            }
            if i + max_jump < n {
                for j in max_reachable..=i + max_jump {
                    if j >= n || s[j] != b'0' {
                        break;
                    }
                    dp[j] = true;
                }
            }
        }
        dp[n - 1]
    }
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        return Self::can_reach_impl(s.as_bytes(), min_jump as usize, max_jump as usize);
    }
}
