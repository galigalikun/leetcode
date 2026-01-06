fn main() {
    assert_eq!(Solution::min_operations(vec![0, 2]), 1);
    assert_eq!(Solution::min_operations(vec![3, 1, 2, 1]), 3);
    assert_eq!(Solution::min_operations(vec![1, 2, 1, 2, 1, 2]), 4);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        return nums.iter().filter(|&&x| x != 0).count() as i32;
    }
}
