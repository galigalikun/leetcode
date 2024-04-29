fn main() {
    assert_eq!(
        Solution::constrained_subset_sum(vec![10, 2, -10, 5, 20], 2),
        37
    );
    assert_eq!(Solution::constrained_subset_sum(vec![-1, -2, -3], 1), -1);
    assert_eq!(
        Solution::constrained_subset_sum(vec![10, -2, -10, -5, 20], 2),
        23
    );
}

struct Solution;
impl Solution {
    pub fn constrained_subset_sum(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![0; n];
        let mut max = 0;
        let mut q = std::collections::VecDeque::new();
        for i in 0..n {
            dp[i] = nums[i];
            if i > 0 {
                dp[i] = std::cmp::max(dp[i], dp[i - 1]);
            }
            if i >= k as usize {
                dp[i] = std::cmp::max(dp[i], max);
            }
            while !q.is_empty() && i as i32 - q.front().unwrap() > k {
                q.pop_front();
            }
            if !q.is_empty() {
                dp[i] = std::cmp::max(dp[i], dp[*q.front().unwrap() as usize]);
            }
            while !q.is_empty() && dp[*q.back().unwrap() as usize] <= dp[i] {
                q.pop_back();
            }
            q.push_back(i as i32);
            max = std::cmp::max(max, dp[i]);
        }
        return max;
    }
}
