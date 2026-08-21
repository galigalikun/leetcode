fn main() {
    assert_eq!(Solution::nth_magical_number(1, 2, 3), 2);
    assert_eq!(Solution::nth_magical_number(4, 2, 3), 6);
}

struct Solution;
impl Solution {
    pub fn nth_magical_number(n: i32, a: i32, b: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let n = n as i64;
        let a = a as i64;
        let b = b as i64;

        let g = Self::gcd(a, b);
        let lcm = a / g * b;

        let mut left = a.min(b);
        let mut right = left * n;

        while left < right {
            let mid = left + (right - left) / 2;
            let count = mid / a + mid / b - mid / lcm;

            if count < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        (left % MOD) as i32
    }

    fn gcd(mut x: i64, mut y: i64) -> i64 {
        while y != 0 {
            let r = x % y;
            x = y;
            y = r;
        }
        x
    }
}
