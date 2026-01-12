fn main() {
    assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
    assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
}

struct Solution;
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        return nums
            .into_iter()
            .filter(|&x| x != 0)
            .collect::<std::collections::HashSet<_>>()
            .len() as i32;
    }
}
