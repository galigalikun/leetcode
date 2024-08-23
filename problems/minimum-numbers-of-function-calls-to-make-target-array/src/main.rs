fn main() {
    assert_eq!(Solution::min_operations(vec![1, 5]), 5);
    assert_eq!(Solution::min_operations(vec![2, 2]), 3);
    assert_eq!(Solution::min_operations(vec![4, 2, 5]), 6);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut nums = nums;
        for n in 1..nums.len() {
            if nums[n] <= nums[n - 1] {
                count += nums[n - 1] - nums[n] + 1;
                nums[n] = nums[n - 1] + 1;
            }
        }
        return count;
    }
}
