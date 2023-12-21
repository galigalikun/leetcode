fn main() {
    assert_eq!(Solution::min_flips(2, 6, 5), 3);
    assert_eq!(Solution::min_flips(4, 2, 7), 1);
    assert_eq!(Solution::min_flips(1, 2, 3), 0);
}

struct Solution;
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        let mut a = a;
        let mut b = b;
        let mut c = c;
        let mut count = 0;
        std::mem::swap(&mut a, &mut c);
        std::mem::swap(&mut b, &mut c);
        while a > 0 || b > 0 || c > 0 {
            let a_bit = a & 1;
            let b_bit = b & 1;
            let c_bit = c & 1;
            if c_bit == 0 {
                count += a_bit + b_bit;
            } else {
                count += (a_bit | b_bit) ^ 1;
            }
            a >>= 1;
            b >>= 1;
            c >>= 1;
        }
        return count;
    }
}
