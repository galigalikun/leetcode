fn main() {
    assert_eq!(Solution::maximum_difference(vec![7, 1, 5, 4]), 4);
    assert_eq!(Solution::maximum_difference(vec![9, 4, 3, 2]), -1);
    assert_eq!(Solution::maximum_difference(vec![1, 5, 2, 10]), 9);
}

struct Solution;
impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        return nums.iter().max().unwrap() - nums.iter().min().unwrap();
    }
}
