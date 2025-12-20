fn main() {
    assert_eq!(
        Solution::remove_anagrams(vec![
            "abba".to_string(),
            "baba".to_string(),
            "bbaa".to_string(),
            "cd".to_string(),
            "cd".to_string()
        ]),
        vec!["abba".to_string(), "cd".to_string()]
    );
    assert_eq!(
        Solution::remove_anagrams(vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string()
        ]),
        vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
            "e".to_string()
        ]
    );
}

struct Solution;
impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        return words.into_iter().fold(Vec::new(), |mut acc, word| {
            if acc.last().map_or(true, |last| {
                let mut w1: Vec<char> = last.chars().collect();
                let mut w2: Vec<char> = word.chars().collect();
                w1.sort_unstable();
                w2.sort_unstable();
                w1 != w2
            }) {
                acc.push(word);
            }
            acc
        });
    }
}
