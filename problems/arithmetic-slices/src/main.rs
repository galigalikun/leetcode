fn main() {
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
    assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
}

struct Solution {}
// https://dev.to/seanpgallivan/arithmetic-slices-4pla
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut diff = 0;
        let mut result = 0;
        let mut count = 0;
        for i in 1..nums.len() {
            let n_diff = nums[i] - nums[i - 1];
            if n_diff == diff {
                result += count;
                count += 1;
            } else {
                diff = n_diff;
                count = 1;
            }
        }
        return result;
    }
}
