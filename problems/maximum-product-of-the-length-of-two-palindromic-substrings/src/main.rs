fn main() {
    assert_eq!(Solution::max_product("ababbb".to_string()), 9);
    assert_eq!(Solution::max_product("zaaaxbbby".to_string()), 9);
}

struct Solution;
impl Solution {
    fn split_palindromic_substrings(s: &str, mask: usize) -> (String, String) {
        let mut first = String::new();
        let mut second = String::new();

        for i in 0..s.len() {
            if (mask & (1 << i)) != 0 {
                first.push(s.chars().nth(i).unwrap());
            } else {
                second.push(s.chars().nth(i).unwrap());
            }
        }

        (first, second)
    }
    pub fn max_product(s: String) -> i64 {
        let n = s.len();
        let mut max_product = 0;

        for i in 1..(1 << n) {
            let (first, second) = Self::split_palindromic_substrings(&s, i);
            max_product = max_product.max(first.len() as i64 * second.len() as i64);
        }

        max_product
    }
}
