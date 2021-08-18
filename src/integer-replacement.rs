fn main() {
    assert_eq!(Solution::integer_replacement(8), 3);
    assert_eq!(Solution::integer_replacement(7), 4);
    assert_eq!(Solution::integer_replacement(1234), 14);
    assert_eq!(Solution::integer_replacement(65535), 17);
    assert_eq!(Solution::integer_replacement(10000), 16);
    assert_eq!(Solution::integer_replacement(2147483647), 32);
}

pub struct Solution {}
// https://www.tutorialspoint.com/integer-replacement-in-cplusplus
impl Solution {
    pub fn integer_replacement(n: i32) -> i32 {
        let mut np: i64 = n as i64;
        let mut c = 0;
        while np != 1 {
            if np % 2 == 0 {
                np = np / 2;
            } else {
                if np == 3 || ((np >> 1) & 1) == 0 {
                    np -= 1;
                } else {
                    np += 1;
                }
            }
            c += 1;
        }

        return c;
    }
}
