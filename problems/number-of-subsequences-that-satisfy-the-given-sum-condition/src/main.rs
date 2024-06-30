fn main() {
    assert_eq!(Solution::num_subseq(vec![3, 5, 6, 7], 9), 4);
    assert_eq!(Solution::num_subseq(vec![3, 3, 6, 8], 10), 6);
    assert_eq!(Solution::num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
}

struct Solution;
impl Solution {
    pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut res = 0;
        let mut left = 0;
        let mut right = nums.len() - 1;
        let mod_num = 1000000007;
        let mut pow = vec![1; nums.len()];
        for i in 1..nums.len() {
            pow[i] = pow[i - 1] * 2 % mod_num;
        }
        while left <= right {
            if nums[left] + nums[right] <= target {
                res = (res + pow[right - left]) % mod_num;
                left += 1;
            } else {
                right -= 1;
            }
        }
        return res as i32;
    }
}
