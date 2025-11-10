fn main() {
    assert_eq!(
        Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4),
        2
    );
    assert_eq!(
        Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 7),
        3
    );
    assert_eq!(
        Solution::min_operations_to_make_median_k(vec![1, 2, 3, 4, 5, 6], 4),
        0
    );
}

struct Solution;
impl Solution {
    pub fn min_operations_to_make_median_k(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        let median_index = n / 2;
        let median = nums[median_index];
        if median < k {
            let mut operations = 0;
            for i in median_index..n {
                if nums[i] < k {
                    operations += (k - nums[i]) as i64;
                }
            }
            return operations;
        } else if median > k {
            let mut operations = 0;
            for i in 0..=median_index {
                if nums[i] > k {
                    operations += (nums[i] - k) as i64;
                }
            }
            return operations;
        } else {
            return 0;
        }
    }
}
