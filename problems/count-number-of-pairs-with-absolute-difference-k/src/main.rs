fn main() {
    assert_eq!(Solution::count_k_difference(vec![1, 2, 2, 1], 1), 4);
    assert_eq!(Solution::count_k_difference(vec![1, 3], 3), 0);
    assert_eq!(Solution::count_k_difference(vec![3, 2, 1, 5, 4], 2), 3);
}

struct Solution;
impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        return nums.iter().enumerate().fold(0, |acc, (i, &n)| {
            acc + nums[i + 1..]
                .iter()
                .filter(|&&x| (n - x).abs() == k)
                .count() as i32
        });
    }
}
