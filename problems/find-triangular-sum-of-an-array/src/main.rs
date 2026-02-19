fn main() {
    assert_eq!(Solution::triangular_sum(vec![1, 2, 3, 4, 5]), 8);
    assert_eq!(Solution::triangular_sum(vec![5]), 5);
}

struct Solution;
impl Solution {
    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        while nums.len() > 1 {
            for i in 0..nums.len() - 1 {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
            nums.pop();
        }
        nums[0]
    }
}
