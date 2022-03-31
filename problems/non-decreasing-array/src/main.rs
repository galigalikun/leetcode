fn main() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
    assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
}

struct Solution {}
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut n = nums[0];
        for i in 1..nums.len() {
            if nums[i] < n {
                if i == 1 || nums[i - 1] <= nums[i] {
                    return true;
                } else {
                    return false;
                }
            }
            n = nums[i];
        }
        return false;
    }
}
