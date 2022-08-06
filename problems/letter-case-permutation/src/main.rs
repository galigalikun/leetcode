use std::ascii::AsciiExt;

fn main() {
    assert_eq!(Solution::letter_case_permutation("a1b2".to_string()), vec!["a1b2","a1B2","A1b2","A1B2"]);
    assert_eq!(Solution::letter_case_permutation("3z4".to_string()), vec!["3z4","3Z4"]);
}

struct Solution{}
impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut ans = vec![1];
        for (i, c) in s.chars().enumerate() {
            if c.is_alphabetic() {
                println!("debug {}", c.to_ascii_uppercase());
            }
        }
        return vec![];
    }
}
