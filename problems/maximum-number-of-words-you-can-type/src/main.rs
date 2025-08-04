fn main() {
    assert_eq!(
        Solution::can_be_typed_words("hello world".to_string(), "ad".to_string()),
        1
    );
    assert_eq!(
        Solution::can_be_typed_words("leet code".to_string(), "lt".to_string()),
        1
    );
    assert_eq!(
        Solution::can_be_typed_words("leet code".to_string(), "e".to_string()),
        0
    );
}

struct Solution;
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        return text
            .split(' ')
            .filter(|word| !word.chars().any(|c| broken_letters.contains(c)))
            .count() as i32;
    }
}
