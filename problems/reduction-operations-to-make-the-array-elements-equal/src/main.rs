fn main() {
    assert_eq!(Solution::reduction_operations(vec![5, 1, 3]), 3);
    assert_eq!(Solution::reduction_operations(vec![1, 1, 1]), 0);
    assert_eq!(Solution::reduction_operations(vec![1, 1, 2, 2, 3]), 4);
}

struct Solution;
impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        return nums.iter().fold(nums.len() as i32, |acc, &x| {
            acc - (nums.iter().filter(|&&y| y < x).count() as i32)
        });
    }
}
