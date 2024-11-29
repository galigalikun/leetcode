fn main() {
    assert_eq!(
        Solution::array_strings_are_equal(
            vec!["ab".to_string(), "c".to_string()],
            vec!["a".to_string(), "bc".to_string()]
        ),
        true
    );
    assert_eq!(
        Solution::array_strings_are_equal(
            vec!["a".to_string(), "cb".to_string()],
            vec!["ab".to_string(), "c".to_string()]
        ),
        false
    );
    assert_eq!(
        Solution::array_strings_are_equal(
            vec!["abc".to_string(), "d".to_string(), "defg".to_string()],
            vec!["abcddefg".to_string()]
        ),
        true
    );
}

struct Solution;
impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        let word1: Vec<char> = word1.join("").chars().collect();
        let word2: Vec<char> = word2.join("").chars().collect();
        return word1 == word2;
    }
}
