fn main() {
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
    assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
}

pub struct Solution {}
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        return nums.into_iter().any(|n| n == target);
    }
}
