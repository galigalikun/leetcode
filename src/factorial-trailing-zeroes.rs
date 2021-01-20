fn main() {
    assert_eq!(Solution::trailing_zeroes(3), 0);
    assert_eq!(Solution::trailing_zeroes(5), 1);
    assert_eq!(Solution::trailing_zeroes(0), 0);
    assert_eq!(Solution::trailing_zeroes(10), 2);
    assert_eq!(Solution::trailing_zeroes(25), 6);
}

pub struct Solution {}
impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        // 0 -> 1
        // 1 -> 1
        // 2 -> 2
        // 3 -> 6
        // 4 -> 24
        // 5 -> 120
        // 6 -> 720
        // 7 -> 5040
        // 8 -> 40320
        // 9 -> 362880
        // 10 -> 3628800
        // 11 -> 39916800
        // 12 -> 479001600
        // 13 -> 6227020800
        // 14 -> 87178291200
        // 15 -> 1307674368000
        // 16 -> 20922789888000
        // 17 -> 355687428096000
        // 18 ->
        // 10000 ->
        // log 5 10000 -> 5.722706232293573
        return n / 3125 + n / 625 + n / 125 + n / 25 + n / 5;
    }
}
