fn main() {
    assert_eq!(
        Solution::number_of_special_chars(String::from("aaAbcBC")),
        3
    );
    assert_eq!(Solution::number_of_special_chars(String::from("abc")), 0);
    assert_eq!(Solution::number_of_special_chars(String::from("abBCab")), 1);
}

struct Solution;
impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower = [false; 26];
        let mut upper = [false; 26];

        for ch in word.chars() {
            if ch.is_ascii_lowercase() {
                lower[(ch as u8 - b'a') as usize] = true;
            } else if ch.is_ascii_uppercase() {
                upper[(ch as u8 - b'A') as usize] = true;
            }
        }

        lower
            .iter()
            .zip(upper.iter())
            .filter(|(lo, up)| **lo && **up)
            .count() as i32
    }
}
