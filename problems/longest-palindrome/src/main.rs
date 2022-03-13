fn main() {
    assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    assert_eq!(Solution::longest_palindrome("bb".to_string()), 2);
}

struct Solution {}
// https://cheonhyangzhang.gitbooks.io/leetcode-solutions/content/409-longest-palindrome.html
use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            if let Some(m) = map.get_mut(&c) {
                *m += 1;
            } else {
                map.insert(c, 1);
            }
        }
        let mut count = 0;
        for (_, v) in map {
            count += v / 2;
        }
        return std::cmp::min(s.len() as i32, count * 2 + 1);
    }
}
