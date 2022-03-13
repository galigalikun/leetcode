fn main() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);

    assert_eq!(Solution::max_profit(vec![1]), 0);
}

struct Solution {}
// https://twchen.gitbook.io/leetcode/dynamic-programming/best-time-to-buy-and-sell-stock-with-cooldown
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        if n == 0 {
            return 0;
        }
        let mut stock = vec![0; n + 1];
        let mut money = vec![0; n + 1];
        stock[1] -= prices[0];
        for i in 1..n {
            stock[i + 1] = std::cmp::max(stock[i], money[i - 1] - prices[i]);
            money[i + 1] = std::cmp::max(stock[i] + prices[i], money[i]);
        }
        return money[n];
    }
}
