fn main() {
    assert_eq!(Solution::min_operations(vec![1, 1, 1]), 3);
    assert_eq!(Solution::min_operations(vec![1, 5, 2, 4, 1]), 14);
    assert_eq!(Solution::min_operations(vec![8]), 0);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut ans = 0;

        for i in 1..nums.len() {
            if nums[i] <= nums[i - 1] {
                let increment = nums[i - 1] - nums[i] + 1;
                nums[i] += increment;
                ans += increment;
            }
        }
        ans
    }
}
