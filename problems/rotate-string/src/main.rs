fn main() {
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "cdeab".to_string()),
        true
    );
    assert_eq!(
        Solution::rotate_string("abcde".to_string(), "abced".to_string()),
        false
    );
}

struct Solution {}
impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        for i in 1..s.len() {
            if format!("{}{}", &s[i..s.len()], &s[0..i]) == goal {
                return true;
            }
        }
        return false;
    }
}
