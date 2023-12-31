fn main() {
    assert_eq!(Solution::min_taps(5, vec![3, 4, 1, 1, 0, 0]), 1);
    assert_eq!(Solution::min_taps(3, vec![0, 0, 0, 0]), -1);
}

struct Solution;
impl Solution {
    pub fn min_taps(n: i32, ranges: Vec<i32>) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        for (i, r) in ranges.iter().enumerate() {
            let left = (i as i32 - r).max(0) as usize;
            let right = (i as i32 + r).min(n) as usize;
            dp[left] = dp[left].max(right);
        }
        let mut right = dp[0];
        let mut next = right;
        let mut ans = 1;
        for i in 1..=n as usize {
            if i > right {
                return -1;
            }
            next = next.max(dp[i]);
            if i == right {
                ans += 1;
                right = next;
            }
        }
        return ans;
    }
}
