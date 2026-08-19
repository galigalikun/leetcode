use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::shortest_subarray(vec![1], 1), 1);
    assert_eq!(Solution::shortest_subarray(vec![1,2], 4), -1);
    assert_eq!(Solution::shortest_subarray(vec![2,-1,2], 3), 3);
}

struct Solution;
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let target = i64::from(k);

        let mut prefix = vec![0_i64; n + 1];
        for idx in 0..n {
            prefix[idx + 1] = prefix[idx] + i64::from(nums[idx]);
        }

        let mut answer = n + 1;
        let mut deque: VecDeque<usize> = VecDeque::new();

        for right in 0..=n {
            while let Some(&left) = deque.front() {
                if prefix[right] - prefix[left] >= target {
                    answer = answer.min(right - left);
                    deque.pop_front();
                } else {
                    break;
                }
            }

            while let Some(&last) = deque.back() {
                if prefix[right] <= prefix[last] {
                    deque.pop_back();
                } else {
                    break;
                }
            }

            deque.push_back(right);
        }

        if answer == n + 1 {
            -1
        } else {
            answer as i32
        }
    }
}
