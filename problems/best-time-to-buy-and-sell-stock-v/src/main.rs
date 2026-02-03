fn main() {
    assert_eq!(Solution::maximum_profit(vec![1, 7, 9, 8, 2], 2), 14);
    assert_eq!(
        Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3),
        36
    );
}

struct Solution;
impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let mut prices = prices;
        prices.sort_unstable_by(|a, b| b.cmp(a));
        prices.iter().take(k as usize).map(|&x| x as i64).sum()
    }
}
