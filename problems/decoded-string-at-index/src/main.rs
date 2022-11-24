fn main() {
    assert_eq!(Solution::decode_at_index("leet2code3".to_string(), 10), "o");
    assert_eq!(Solution::decode_at_index("ha22".to_string(), 5), "h");
    assert_eq!(Solution::decode_at_index("a2345678999999999999999".to_string(), 1), "a");
}

struct Solution;
impl Solution {
    pub fn decode_at_index(s: String, k: i32) -> String {
        let mut pref = String::new();
        for c in s.chars() {
            if c.is_numeric() {
                pref = pref.clone().repeat(c.to_digit(10).unwrap() as usize);
            } else {
                pref = format!("{}{}", pref, c);
            }
        }
        return pref.chars().nth(k as usize - 1).unwrap().to_string();
    }
}
