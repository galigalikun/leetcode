fn main() {
    assert_eq!(Solution::min_operations(vec![4, 2, 5, 3]), 0);
    assert_eq!(Solution::min_operations(vec![1, 2, 3, 5, 6]), 1);
    assert_eq!(Solution::min_operations(vec![1, 10, 100, 1000]), 3);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        return nums.iter().max().unwrap() - nums.iter().min().unwrap();
    }
}
