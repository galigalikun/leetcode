fn main() {
    assert_eq!(
        Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4),
        12.75000
    );
    assert_eq!(Solution::find_max_average(vec![5], 1), 5.00000);
    assert_eq!(Solution::find_max_average(vec![1, 0, 1, 4, 2], 4), 1.75000);
}

struct Solution {}
impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut n = std::i32::MIN;
        for i in 0..=nums.len() - k as usize {
            n = std::cmp::max(n, *(&nums[i..i + k as usize].iter().fold(0, |a, b| a + b)));
        }
        return n as f64 / k as f64;
    }
}
