fn main() {
    assert_eq!(Solution::is_trionic(vec![1, 3, 5, 4, 2, 6]), true);
    assert_eq!(Solution::is_trionic(vec![2, 1, 3]), false);
}

struct Solution;
impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        return nums.len() == 3 && nums[0] + nums[1] + nums[2] == 0;
    }
}
