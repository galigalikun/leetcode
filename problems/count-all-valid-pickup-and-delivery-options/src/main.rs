fn main() {
    assert_eq!(Solution::count_orders(1), 1);
    assert_eq!(Solution::count_orders(2), 6);
    assert_eq!(Solution::count_orders(3), 90);
    assert_eq!(Solution::count_orders(8), 729647433);
}

struct Solution;
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        return (1..=n).fold(1, |acc, x| acc * (2 * x - 1) * x % 1_000_000_007);
    }
}
