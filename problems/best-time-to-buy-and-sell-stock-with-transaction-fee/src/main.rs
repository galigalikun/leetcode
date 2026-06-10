fn main() {
    assert_eq!(Solution::max_profit(vec![1,3,2,8,4,9], 2), 8);
    assert_eq!(Solution::max_profit(vec![1,3,7,5,10,3], 3), 6);
}
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        if prices.is_empty() {
            return 0;
        }

        // `cash`: max profit when not holding stock after current day.
        // `hold`: max profit when holding stock after current day.
        let mut cash = 0;
        let mut hold = -prices[0];

        for &price in prices.iter().skip(1) {
            let prev_cash = cash;
            cash = std::cmp::max(cash, hold + price - fee);
            hold = std::cmp::max(hold, prev_cash - price);
        }

        cash
    }
}
