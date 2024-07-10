fn main() {
    assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
    assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
    assert_eq!(Solution::min_difference(vec![3, 100, 20]), 0);
}

struct Solution;
impl Solution {
    pub fn min_difference(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let n = nums.len();
        if n <= 3 {
            return 0;
        }
        let mut res = i32::MAX;
        for i in 0..4 {
            res = res.min(nums[n - 4 + i] - nums[i]);
        }
        return res;
    }
}
