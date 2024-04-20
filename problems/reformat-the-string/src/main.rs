fn main() {
    assert_eq!(Solution::reformat("a0b1c2".to_string()), "a0b1c2");
    assert_eq!(Solution::reformat("leetcode".to_string()), "");
    assert_eq!(Solution::reformat("1229857369".to_string()), "");
}

struct Solution;
impl Solution {
    pub fn reformat(s: String) -> String {
        let mut digits = Vec::new();
        let mut letters = Vec::new();
        for c in s.chars() {
            if c.is_digit(10) {
                digits.push(c);
            } else {
                letters.push(c);
            }
        }
        if (digits.len() as i32 - letters.len() as i32).abs() > 1 {
            return "".to_string();
        }
        let mut s = String::new();
        let mut i = 0;
        let mut j = 0;
        if digits.len() > letters.len() {
            s.push(digits[i]);
            i += 1;
        }
        while i < digits.len() || j < letters.len() {
            if j < letters.len() {
                s.push(letters[j]);
                j += 1;
            }
            if i < digits.len() {
                s.push(digits[i]);
                i += 1;
            }
        }
        return s;
    }
}
