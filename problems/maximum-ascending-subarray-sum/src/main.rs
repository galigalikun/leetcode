fn main() {
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    assert_eq!(
        Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
        33
    );
}

struct Solution;
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        return nums
            .split(|&x| x <= 0)
            .map(|slice| slice.iter().fold(0, |acc, &x| acc + x))
            .max()
            .unwrap_or(0);
    }
}
