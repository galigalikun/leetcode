fn main() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
    assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
}

struct Solution {}
// https://dev.to/seanpgallivan/solution-combination-sum-iv-3620
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp = vec![0; target as usize + 1];
        dp[0] = 1;
        for i in 1..=target {
            for n in nums.iter().filter(|&n| n <= &i) {
                dp[i as usize] += dp[(i - n) as usize];
            }
        }

        return dp[target as usize];
    }
}
