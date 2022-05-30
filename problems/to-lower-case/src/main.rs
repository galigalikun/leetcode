fn main() {
    assert_eq!(Solution::to_lower_case("Hello".to_string()), "hello");
    assert_eq!(Solution::to_lower_case("here".to_string()), "here");
    assert_eq!(Solution::to_lower_case("LOVELY".to_string()), "lovely");
}

struct Solution {}
impl Solution {
    pub fn to_lower_case(s: String) -> String {
        let mut res = String::new();
        for c in s.chars() {
            if c >= 'A' && c <= 'Z' {
                res.push((c as u8 + 32) as char);
            } else {
                res.push(c);
            }
        }
        return res;
    }
}
