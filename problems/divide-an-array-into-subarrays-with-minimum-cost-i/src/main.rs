fn main() {
    assert_eq!(Solution::minimum_cost(vec![1, 2, 3, 12]), 6);
    assert_eq!(Solution::minimum_cost(vec![5, 4, 3]), 12);
    assert_eq!(Solution::minimum_cost(vec![10, 3, 1, 1]), 12);
}

struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        for i in (0..nums.len()).rev() {
            if (nums.len() - 1 - i) % 3 != 2 {
                ans += nums[i];
            }
        }
        ans
    }
}
