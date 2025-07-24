fn main() {
    assert_eq!(Solution::count_good_numbers(1), 5);
    assert_eq!(Solution::count_good_numbers(4), 400);
    assert_eq!(Solution::count_good_numbers(50), 564908303);
}

struct Solution;
impl Solution {
    fn mod_pow(base: i64, exp: i64, mod_val: i64) -> i64 {
        let mut result = 1;
        let mut b = base % mod_val;
        let mut e = exp;

        while e > 0 {
            if e % 2 == 1 {
                result = (result * b) % mod_val;
            }
            b = (b * b) % mod_val;
            e /= 2;
        }
        result
    }
    pub fn count_good_numbers(n: i64) -> i32 {
        let mod_val = 1_000_000_007;
        if n == 1 {
            return 5;
        }
        let half = n / 2;
        let even_count = (5 * Self::mod_pow(4, half, mod_val)) % mod_val;
        let odd_count = (5 * Self::mod_pow(5, half + n % 2, mod_val)) % mod_val;
        ((even_count * odd_count) % mod_val) as i32
    }
}
