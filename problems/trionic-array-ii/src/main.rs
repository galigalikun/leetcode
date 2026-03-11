fn main() {
    assert_eq!(Solution::max_sum_trionic(vec![0,-2,-1,-3,0,2,-1]), -4);
    assert_eq!(Solution::max_sum_trionic(vec![1,4,2,7]), 14);
}

struct Solution;
impl Solution {
    pub fn max_sum_trionic(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut ans = 0;
        for i in (0..n).step_by(3) {
            ans += nums[i] as i64;
        }
        ans
    }
}
