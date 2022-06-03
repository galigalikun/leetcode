fn main() {
    assert_eq!(Solution::max_profit(vec![1,3,2,8,4,9], 2), 8);
    assert_eq!(Solution::max_profit(vec![1,3,7,5,10,3], 3), 6);
}
struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut buy = 0;
        let mut sell = 0;
        let mut hold = 0;
        for i in 0..prices.len() {
            buy = std::cmp::max(buy, hold - prices[i]);
            hold = std::cmp::max(hold, sell + prices[i] - fee);
            sell = std::cmp::max(sell, buy + prices[i]);
        }
        return sell;
    }
}
