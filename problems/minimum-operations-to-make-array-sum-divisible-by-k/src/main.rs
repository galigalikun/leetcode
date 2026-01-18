fn main() {
    assert_eq!(Solution::min_operations(vec![3, 9, 7], 5), 4);
    assert_eq!(Solution::min_operations(vec![4, 1, 3], 4), 0);
    assert_eq!(Solution::min_operations(vec![3, 2], 6), 5);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        let a = nums[n-1] - k;
        nums[n-1] = k;
        let b = nums.iter().map(|&x| if x > a { x - a } else { 0 }).sum::<i32>();
        return std::cmp::min(a, b);
    }
}
