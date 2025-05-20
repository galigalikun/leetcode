fn main() {
    assert_eq!(Solution::split_string("1234".to_string()), false);
    assert_eq!(Solution::split_string("050043".to_string()), true);
    assert_eq!(Solution::split_string("9080701".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn split_string_helper(s: &str, start: usize, prev: i64) -> bool {
        if start == s.len() {
            return true;
        }
        let mut num = 0;
        for i in start..s.len() {
            num = num * 10 + (s.as_bytes()[i] - b'0') as i64;
            if num > prev + 1 {
                break;
            }
            if num == prev + 1 && Self::split_string_helper(s, i + 1, num) {
                return true;
            }
        }
        false
    }
    pub fn split_string(s: String) -> bool {
        return Self::split_string_helper(&s, 0, 0);
    }
}
