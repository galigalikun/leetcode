fn main() {
    assert_eq!(
        Solution::longest_word(vec![
            "w".to_string(),
            "wo".to_string(),
            "wor".to_string(),
            "worl".to_string(),
            "world".to_string()
        ]),
        "world"
    );
    assert_eq!(
        Solution::longest_word(vec![
            "a".to_string(),
            "banana".to_string(),
            "app".to_string(),
            "appl".to_string(),
            "ap".to_string(),
            "apply".to_string(),
            "apple".to_string()
        ]),
        "apple"
    );
}

struct Solution {}
impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut words = words;
        words.sort_by(|a, b| b.len().cmp(&a.len()));
        let mut res = String::new();
        for word in words {
            if word.chars().all(|c| res.contains(c)) {
                res = word;
            }
        }
        return res;
    }
}
