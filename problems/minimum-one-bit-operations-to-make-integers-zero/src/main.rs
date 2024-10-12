fn main() {
    assert_eq!(Solution::minimum_one_bit_operations(3), 2);
    assert_eq!(Solution::minimum_one_bit_operations(6), 4);
}

struct Solution;
impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        let mut n = n;
        let mut res = 0;
        while n > 0 {
            res ^= n;
            n >>= 1;
        }
        res
    }
}
