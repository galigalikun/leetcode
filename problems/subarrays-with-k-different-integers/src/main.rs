fn main() {
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 2), 7);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 3, 4], 3), 3);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 3, 1, 2], 2), 4);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![1, 2, 1, 2, 3], 1), 5);
    assert_eq!(Solution::subarrays_with_k_distinct(vec![], 0), 0);
}

struct Solution;
impl Solution {
    fn at_most_k_distinct(nums: &[i32], k: i32) -> i32 {
        if k <= 0 {
            return 0;
        }

        let mut freq = std::collections::HashMap::new();
        let mut left = 0;
        let mut total = 0;

        for right in 0..nums.len() {
            *freq.entry(nums[right]).or_insert(0) += 1;

            while freq.len() > k as usize {
                let value = nums[left];
                let count = freq.get_mut(&value).unwrap();
                *count -= 1;

                if *count == 0 {
                    freq.remove(&value);
                }

                left += 1;
            }

            total += (right - left + 1) as i32;
        }

        total
    }

    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 0 || nums.is_empty() {
            return 0;
        }

        Self::at_most_k_distinct(&nums, k) - Self::at_most_k_distinct(&nums, k - 1)
    }
}
