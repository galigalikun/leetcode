fn main() {
    assert_eq!(Solution::median_of_uniqueness_array(vec![1, 2, 3]), 1);
    assert_eq!(Solution::median_of_uniqueness_array(vec![3, 4, 3, 4, 5]), 2);
    assert_eq!(Solution::median_of_uniqueness_array(vec![4, 3, 5, 4]), 2);
}

struct Solution;
impl Solution {
    pub fn median_of_uniqueness_array(nums: Vec<i32>) -> i32 {
        let mut unique_nums: Vec<i32> = nums
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        unique_nums.sort_unstable();
        let len = unique_nums.len();
        if len % 2 == 1 {
            return unique_nums[len / 2];
        }
        return 0;
    }
}
