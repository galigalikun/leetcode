fn main() {
    assert_eq!(Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2), 10);
    assert_eq!(Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2), 9);
}

struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let n = prices.len();
        let mut dp = vec![vec![vec![0i64; 2]; (k + 1) as usize]; n + 1];
        for i in (0..n).rev() {
            for j in 0..=k as usize {
                for s in 0..2 {
                    if s == 0 {
                        let mut buy = dp[i + 1][j][0];
                        if j < k as usize {
                            buy = buy
                                .max(dp[i + 1][j + 1][1] + prices[i] as i64 + strategy[i] as i64);
                        }
                        dp[i][j][s] = buy;
                    } else {
                        let mut sell = dp[i + 1][j][1];
                        sell = sell.max(dp[i + 1][j][0] - prices[i] as i64 + strategy[i] as i64);
                        dp[i][j][s] = sell;
                    }
                }
            }
        }
        dp[0][0][0]
    }
}
