fn main() {
    assert_eq!(Solution::min_removal(vec![2, 1, 5], 2), 1);
    assert_eq!(Solution::min_removal(vec![1, 6, 2, 9], 3), 2);
    assert_eq!(Solution::min_removal(vec![4, 6], 2), 0);
    assert_eq!(Solution::min_removal(vec![2, 12], 2), 1);
}

struct Solution;
impl Solution {
    pub fn min_removal(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut i = 0;
        for j in 0..nums.len() {
            while nums[j] - nums[i] > k {
                i += 1;
            }
            if j - i + 1 > 1 {
                return (nums.len() - (j - i + 1)) as i32;
            }
        }
        return 0;
    }
}
