fn main() {
    assert_eq!(Solution::is_valid("aabcbc".to_string()), true);
    assert_eq!(Solution::is_valid("abcabcababcc".to_string()), true);
    assert_eq!(Solution::is_valid("abccba".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                'a' => stack.push('a'),
                'b' => {
                    if stack.pop() != Some('a') {
                        return false;
                    }
                    stack.push('b');
                }
                'c' => {
                    if stack.pop() != Some('b') {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}
