fn main() {
    assert_eq!(Solution::divide(10, 3), 3);
    assert_eq!(Solution::divide(7, -3), -2);
    assert_eq!(Solution::divide(0, 1), 0);
    assert_eq!(Solution::divide(1, 1), 1);
    assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
}

pub struct Solution {}
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if let Some(d) = dividend.checked_div(divisor) {
            return d;
        }
        return i32::max_value();
    }
}
