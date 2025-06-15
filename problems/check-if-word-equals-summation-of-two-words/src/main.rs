fn main() {
    assert_eq!(
        Solution::is_sum_equal("acb".to_string(), "cba".to_string(), "cdb".to_string()),
        true
    );
    assert_eq!(
        Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aab".to_string()),
        false
    );
    assert_eq!(
        Solution::is_sum_equal("aaa".to_string(), "a".to_string(), "aaaa".to_string()),
        true
    );
}

struct Solution;
impl Solution {
    fn word_to_number(word: &str) -> i32 {
        let mut number = 0;
        let mut multiplier = 1;
        for c in word.chars().rev() {
            number += (c as u8 - b'a') as i32 * multiplier;
            multiplier *= 10;
        }
        number
    }
    pub fn is_sum_equal(first_word: String, second_word: String, target_word: String) -> bool {
        return Self::word_to_number(&first_word) + Self::word_to_number(&second_word)
            == Self::word_to_number(&target_word);
    }
}
