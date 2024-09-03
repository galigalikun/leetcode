fn main() {
    assert_eq!(Solution::num_of_ways(vec![2, 1, 3]), 1);
    assert_eq!(Solution::num_of_ways(vec![3, 4, 5, 1, 2]), 5);
    assert_eq!(Solution::num_of_ways(vec![1, 2, 3]), 0);
}

struct Solution;
impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut dp = vec![vec![0; nums.len()]; nums.len()];
        for i in 0..nums.len() {
            for j in 0..nums.len() {
                if i == j {
                    dp[i][j] = 1;
                }
            }
        }
        for i in (0..nums.len()).rev() {
            for j in (0..nums.len()).rev() {
                if i < j {
                    let mut ans = 0;
                    for k in i + 1..nums.len() {
                        if nums[i] < nums[k] {
                            ans += dp[k][j];
                        }
                    }
                    for k in 0..j {
                        if nums[i] < nums[k] {
                            ans += dp[i][k];
                        }
                    }
                    dp[i][j] = ans % 1_000_000_007;
                }
            }
        }
        let mut ans = 0;
        for i in 0..nums.len() {
            ans += dp[i][0];
        }
        ans % 1_000_000_007
    }
}
