fn main() {
    assert_eq!(Solution::nth_ugly_number(3, 2, 3, 5), 4);
    assert_eq!(Solution::nth_ugly_number(4, 2, 3, 4), 6);
    assert_eq!(Solution::nth_ugly_number(5, 2, 11, 13), 10);
}

struct Solution;
impl Solution {
    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        return a;
    }
    pub fn nth_ugly_number(n: i32, a: i32, b: i32, c: i32) -> i32 {
        let a = a as i64;
        let b = b as i64;
        let c = c as i64;
        let n = n as i64;
        let ab = a * b / Self::gcd(a, b);
        let ac = a * c / Self::gcd(a, c);
        let bc = b * c / Self::gcd(b, c);
        let abc = a * bc / Self::gcd(a, bc);
        let mut left = 1;
        let mut right = 2 * 10i64.pow(9);
        while left < right {
            let mid = left + (right - left) / 2;
            let count = mid / a + mid / b + mid / c - mid / ab - mid / ac - mid / bc + mid / abc;
            if count < n {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        return left as i32;
    }
}
