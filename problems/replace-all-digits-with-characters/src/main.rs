fn main() {
    assert_eq!(
        Solution::replace_digits("a1c1e1".to_string()),
        "abcdef".to_string()
    );
    assert_eq!(
        Solution::replace_digits("a1b2c3d4e".to_string()),
        "abbdcfdhe".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn replace_digits(s: String) -> String {
        return s
            .chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 1 {
                    let prev = s.chars().nth(i - 1).unwrap();
                    (c as u8 + prev as u8 - b'0') as char
                } else {
                    c
                }
            })
            .collect();
    }
}
