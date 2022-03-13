fn main() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(Solution::max_profit(vec![]), 0);
}

struct Solution {}
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        if prices.len() > 0 {
            for i in 0..prices.len() - 1 {
                for j in i + 1..prices.len() {
                    let diff = prices[j] - prices[i];
                    if diff > max {
                        max = diff;
                    }
                }
            }
        }

        return max;
    }
}
