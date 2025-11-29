fn main() {
    assert_eq!(Solution::is_valid("234Adas".to_string()), true);
    assert_eq!(Solution::is_valid("b3".to_string()), false);
    assert_eq!(Solution::is_valid("a3$e".to_string()), false);
    assert_eq!(Solution::is_valid("aya".to_string()), true);
    assert_eq!(Solution::is_valid("HsQ".to_string()), false);
    assert_eq!(Solution::is_valid("AhI".to_string()), true);
    assert_eq!(Solution::is_valid("#zwI".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_valid(word: String) -> bool {
        let mut vowel_count = 0;
        let mut consonant_count = 0;
        let mut number_count = 0;
        for c in word.chars() {
            if c.is_ascii_alphabetic() {
                match c.to_ascii_uppercase() {
                    'A' | 'E' | 'I' | 'O' | 'U' => vowel_count += 1,
                    _ => consonant_count += 1,
                }
            } else if c.is_ascii_digit() {
                number_count += 1;
            } else {
                return false;
            }
        }
        let total_count = vowel_count + consonant_count + number_count;
        if vowel_count > 0 && consonant_count > 0 && total_count >= 3 {
            return true;
        }
        return false;
    }
}
