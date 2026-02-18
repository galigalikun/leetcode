fn main() {
    assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    assert_eq!(Solution::largest_perimeter(vec![1, 2, 1, 10]), 0);
}

struct Solution;
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by(|a, b| b.cmp(a));
        for i in 0..nums.len() - 2 {
            if nums[i] < nums[i + 1] + nums[i + 2] {
                return nums[i] + nums[i + 1] + nums[i + 2];
            }
        }
        0
    }
}
