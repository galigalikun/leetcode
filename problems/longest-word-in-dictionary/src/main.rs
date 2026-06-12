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

use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn longest_word(words: Vec<String>) -> String {
        let mut words = words;
        words.sort();

        let mut buildable_words = HashSet::from([String::new()]);
        let mut longest_word = String::new();

        for word in words {
            let prefix = &word[..word.len() - 1];

            if buildable_words.contains(prefix) {
                if word.len() > longest_word.len() {
                    longest_word = word.clone();
                }

                buildable_words.insert(word);
            }
        }

        longest_word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_longest_buildable_word() {
        let words = vec!["w", "wo", "wor", "worl", "world"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = Solution::longest_word(words);

        assert_eq!(result, "world");
    }

    #[test]
    fn prefers_lexicographically_smaller_word_on_tie() {
        let words = vec!["a", "banana", "app", "appl", "ap", "apply", "apple"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = Solution::longest_word(words);

        assert_eq!(result, "apple");
    }

    #[test]
    fn returns_empty_string_when_no_word_is_buildable() {
        let words = vec!["abc", "bc", "ab"]
            .into_iter()
            .map(String::from)
            .collect();

        let result = Solution::longest_word(words);

        assert_eq!(result, "");
    }
}
