fn main() {
    assert_eq!(Solution::max_product("leetcodecom".to_string()), 9);
    assert_eq!(Solution::max_product("bb".to_string()), 1);
    assert_eq!(Solution::max_product("accbcaxxcxx".to_string()), 23);
}

struct Solution;
impl Solution {
    pub fn max_product(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut masks = vec![0; n];
        for i in 0..n {
            masks[i] = 1 << (s[i] - b'a');
            if i > 0 {
                masks[i] |= masks[i - 1];
            }
        }
        let mut max_product = 0;
        for i in 0..n {
            for j in i + 1..n {
                if masks[i] & (masks[n - 1] ^ masks[j - 1]) == 0 {
                    max_product = max_product.max((i + 1) * (n - j));
                }
            }
        }
        max_product as i32
    }
}
