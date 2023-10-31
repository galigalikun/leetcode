fn main() {
    assert_eq!(Solution::max_sum_div_three(vec![3, 6, 5, 1, 8]), 18);
    assert_eq!(Solution::max_sum_div_three(vec![4]), 0);
    assert_eq!(Solution::max_sum_div_three(vec![1, 2, 3, 4, 4]), 12);
}

struct Solution;
impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0, 0, 0];
        for n in nums {
            let mut dp2 = dp.clone();
            for i in 0..3 {
                let m = ((dp[i] + n) % 3) as usize;
                dp2[m] = std::cmp::max(dp2[m], dp[i] + n);
            }
            dp = dp2;
        }
        return dp[0];
    }
}
