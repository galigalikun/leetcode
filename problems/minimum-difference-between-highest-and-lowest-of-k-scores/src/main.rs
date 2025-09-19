fn main() {
    assert_eq!(Solution::minimum_difference(vec![90], 1), 0);
    assert_eq!(Solution::minimum_difference(vec![9, 4, 1, 7], 2), 2);
}

struct Solution;
impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        return if k == 1 {
            0
        } else {
            let mut nums = nums;
            nums.sort_unstable();
            let k = k as usize;
            let mut ans = i32::MAX;
            for i in 0..=nums.len() - k {
                ans = ans.min(nums[i + k - 1] - nums[i]);
            }
            ans
        };
    }
}
