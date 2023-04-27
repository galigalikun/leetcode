fn main() {
    assert_eq!(Solution::bitwise_complement(5), 2);
    assert_eq!(Solution::bitwise_complement(7), 0);
    assert_eq!(Solution::bitwise_complement(10), 5);
}

struct Solution;
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let mut n = n;
        let mut i = 0;
        while n > 0 {
            n >>= 1;
            i += 1;
        }
        let mut n = 0;
        for _ in 0..i {
            n <<= 1;
            n += 1;
        }
        return n - n;
    }
}
