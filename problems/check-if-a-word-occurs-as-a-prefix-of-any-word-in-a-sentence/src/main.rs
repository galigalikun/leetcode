fn main() {
    assert_eq!(
        Solution::is_prefix_of_word("i love eating burger".to_string(), "burg".to_string()),
        4
    );
    assert_eq!(
        Solution::is_prefix_of_word(
            "this problem is an easy problem".to_string(),
            "pro".to_string()
        ),
        2
    );
    assert_eq!(
        Solution::is_prefix_of_word("i am tired".to_string(), "you".to_string()),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        return sentence
            .split_whitespace()
            .position(|word| word.starts_with(&search_word))
            .map_or(-1, |i| i as i32 + 1);
    }
}
