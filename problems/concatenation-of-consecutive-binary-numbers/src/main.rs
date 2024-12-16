fn main() {
    assert_eq!(Solution::concatenated_binary(1), 1);
    assert_eq!(Solution::concatenated_binary(3), 27);
    assert_eq!(Solution::concatenated_binary(12), 505379714);
}

struct Solution;
impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut res = 0;
        let mut len = 0;
        let mod_val = 1_000_000_007;
        for i in 1..=n as i128 {
            len = len + i.leading_zeros() - 1;
            res = ((res << len) + i) % mod_val;
        }
        res as i32
    }
}
