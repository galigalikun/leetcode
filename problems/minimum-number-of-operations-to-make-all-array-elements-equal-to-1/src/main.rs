fn main() {
    assert_eq!(Solution::min_operations(vec![2, 6, 3, 4]), 4);
    assert_eq!(Solution::min_operations(vec![2, 10, 6, 14]), -1);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        return nums.iter().filter(|&&x| x != 0).count() as i32;
    }
}
