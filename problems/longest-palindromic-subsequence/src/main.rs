fn main() {
    assert_eq!(Solution::longest_palindrome_subseq("bbbab".to_string()), 4);
    assert_eq!(Solution::longest_palindrome_subseq("cbbd".to_string()), 2);
    assert_eq!(Solution::longest_palindrome_subseq("aabaa".to_string()), 5);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            if let Some(m) = map.get_mut(&c) {
                *m += 1;
            } else {
                map.insert(c, 1);
            }
        }

        let mut cnt = 0;
        for (_, v) in map {
            cnt = std::cmp::max(cnt, v);
        }

        return cnt;
    }
}
