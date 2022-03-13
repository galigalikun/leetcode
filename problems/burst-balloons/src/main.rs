fn main() {
    assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    assert_eq!(Solution::max_coins(vec![1, 5]), 10);
}

struct Solution {}
// https://www.geeksforgeeks.org/burst-balloon-to-maximize-coins/
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut guard = vec![1];
        guard.extend(nums);
        guard.push(1);
        let mut dp = vec![vec![0; n + 2]; n + 2];
        for length in 1..=n {
            for left in 1..=n - length + 1 {
                let right = left + length - 1;
                for last in left..=right {
                    dp[left][right] = std::cmp::max(
                        dp[left][right],
                        dp[left][last - 1]
                            + guard[left - 1] * guard[last] * guard[right + 1]
                            + dp[last + 1][right],
                    );
                }
            }
        }

        return dp[1][n];
    }
}
