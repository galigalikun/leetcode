fn main() {
    assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    assert_eq!(Solution::my_pow(2.10000, 3), 9.26100);
    assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
    // assert_eq!(Solution::my_pow(1.00012, 1024), 1.130740387);
}

pub struct Solution {}
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1.0;
        }
        if n % 2 == 0 {
            return Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2);
        } else if n > 0 {
            return Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2) * x;
        }
        return Solution::my_pow(x, n / 2) * Solution::my_pow(x, n / 2) / x;
    }
}
