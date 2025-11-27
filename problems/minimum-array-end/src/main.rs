fn main() {
    assert_eq!(Solution::min_end(3, 4), 6);
    assert_eq!(Solution::min_end(2, 7), 15);
}

struct Solution;
impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        return ((n as i64 + x as i64 - 1) / x as i64) * x as i64;
    }
}
