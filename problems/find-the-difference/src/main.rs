fn main() {
    assert_eq!(
        Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
        'e'
    );
    assert_eq!(
        Solution::find_the_difference("".to_string(), "y".to_string()),
        'y'
    );
    assert_eq!(
        Solution::find_the_difference("a".to_string(), "aa".to_string()),
        'a'
    );
    assert_eq!(
        Solution::find_the_difference("ae".to_string(), "aea".to_string()),
        'a'
    );
}

pub struct Solution {}
impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut ss = s;
        for c in t.chars() {
            if let Some(idx) = ss.chars().position(|x| x == c) {
                ss.remove(idx);
            } else {
                return c;
            }
        }
        return '0';
    }
}
