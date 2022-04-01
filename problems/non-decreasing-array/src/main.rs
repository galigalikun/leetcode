fn main() {
    assert_eq!(Solution::check_possibility(vec![4, 2, 3]), true);
    assert_eq!(Solution::check_possibility(vec![4, 2, 1]), false);
}

// https://dev.to/seanpgallivan/solution-non-decreasing-array-1m5c
struct Solution {}
impl Solution {
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut err = 0;
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                if err > 0
                    || (i > 1
                        && i < nums.len() - 1
                        && nums[i - 2] > nums[i]
                        && nums[i + 1] < nums[i - 1])
                {
                    return false;
                }
                err += 1;
            }
        }
        return true;
    }
}
