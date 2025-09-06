fn main() {
    assert_eq!(Solution::min_non_zero_product(1), 1);
    assert_eq!(Solution::min_non_zero_product(2), 6);
    assert_eq!(Solution::min_non_zero_product(3), 1512);
}

struct Solution;
impl Solution {
    pub fn min_non_zero_product(p: i32) -> i32 {
        return ((1i64 << p) - 1) as i32 - 2 * ((1i64 << p) - 2) as i32 % 1_000_000_007;
    }
}
