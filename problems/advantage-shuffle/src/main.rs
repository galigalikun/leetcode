use std::collections::VecDeque;

fn main() {
    assert_eq!(
        Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
        vec![2, 11, 7, 15]
    );
    assert_eq!(
        Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
        vec![24, 32, 8, 12]
    );
}

struct Solution;
impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.is_empty() {
            return Vec::new();
        }

        let mut sorted_nums1 = nums1;
        sorted_nums1.sort_unstable();
        let mut candidates: VecDeque<i32> = VecDeque::from(sorted_nums1);

        let mut indexed_nums2: Vec<(usize, i32)> = nums2.into_iter().enumerate().collect();
        indexed_nums2.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        let mut ans = vec![0; indexed_nums2.len()];
        for (index, value) in indexed_nums2 {
            let largest = *candidates.back().expect("candidates must not be empty");
            ans[index] = if largest > value {
                candidates.pop_back().expect("candidates must not be empty")
            } else {
                candidates
                    .pop_front()
                    .expect("candidates must not be empty")
            };
        }

        ans
    }
}
