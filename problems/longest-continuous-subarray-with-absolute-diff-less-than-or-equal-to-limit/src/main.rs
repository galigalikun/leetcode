fn main() {
    assert_eq!(Solution::longest_subarray(vec![8, 2, 4, 7], 4), 2);
    assert_eq!(Solution::longest_subarray(vec![10, 1, 2, 4, 7, 2], 5), 4);
    assert_eq!(
        Solution::longest_subarray(vec![4, 2, 2, 2, 4, 4, 2, 2], 0),
        3
    );
}

struct Solution;
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        use std::collections::VecDeque;
        let mut min_deque = VecDeque::new();
        let mut max_deque = VecDeque::new();
        let mut left = 0;
        let mut right = 0;
        let mut res = 0;
        while right < nums.len() {
            while !min_deque.is_empty() && nums[right] < nums[*min_deque.back().unwrap()] {
                min_deque.pop_back();
            }
            while !max_deque.is_empty() && nums[right] > nums[*max_deque.back().unwrap()] {
                max_deque.pop_back();
            }
            min_deque.push_back(right);
            max_deque.push_back(right);
            while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > limit {
                if left == *min_deque.front().unwrap() {
                    min_deque.pop_front();
                }
                if left == *max_deque.front().unwrap() {
                    max_deque.pop_front();
                }
                left += 1;
            }
            res = res.max(right - left + 1);
            right += 1;
        }
        return res as i32;
    }
}
