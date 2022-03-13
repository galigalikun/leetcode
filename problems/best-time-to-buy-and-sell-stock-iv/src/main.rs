fn main() {
    assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    assert_eq!(Solution::max_profit(1, vec![1, 2]), 1);
}

struct Solution {}
// https://medium.com/@yzhua3/leetcode-best-time-to-buy-and-sell-stock-iv-bf226251d37
impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if k as usize > n / 2 {
            let mut res = 0;
            for i in 1..n {
                res += std::cmp::max(0, prices[i] - prices[i - 1]);
            }
            return res;
        }
        let mut dp = vec![vec![0; n]; k as usize + 1];
        for i in 1..=k as usize {
            let mut profit = -1 * prices[0];
            for j in 1..n {
                dp[i][j] = std::cmp::max(dp[i][j - 1], profit + prices[j]);
                profit = std::cmp::max(profit, dp[i - 1][j - 1] - prices[j]);
            }
        }
        return dp[k as usize][n - 1];
    }
}
