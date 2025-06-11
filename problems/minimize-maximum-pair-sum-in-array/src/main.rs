fn main() {
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
    assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
}

struct Solution;
impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let n = nums.len();
        (0..n / 2).map(|i| nums[i] + nums[n - 1 - i]).max().unwrap()
    }
}
