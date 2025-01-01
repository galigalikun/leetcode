fn main() {
    assert_eq!(Solution::max_result(vec![1, -1, -2, 4, -7, 3], 2), 7);
    assert_eq!(Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3), 17);
    assert_eq!(
        Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2),
        0
    );
}

struct Solution;
impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        let mut dp = vec![0; n];
        dp[0] = nums[0];
        let mut deque = std::collections::VecDeque::new();
        deque.push_back(0);
        for i in 1..n {
            if i > k {
                while !deque.is_empty() && *deque.front().unwrap() < i - k {
                    deque.pop_front();
                }
            }
            dp[i] = dp[*deque.front().unwrap()] + nums[i];
            while !deque.is_empty() && dp[*deque.back().unwrap()] <= dp[i] {
                deque.pop_back();
            }
            deque.push_back(i);
        }
        dp[n - 1]
    }
}
