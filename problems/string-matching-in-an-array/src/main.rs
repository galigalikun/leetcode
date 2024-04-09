fn main() {
    assert_eq!(
        Solution::string_matching(vec![
            "mass".to_string(),
            "as".to_string(),
            "hero".to_string(),
            "superhero".to_string()
        ]),
        vec!["as", "hero"]
    );
    assert_eq!(
        Solution::string_matching(vec![
            "leetcode".to_string(),
            "et".to_string(),
            "code".to_string()
        ]),
        vec!["et", "code"]
    );
    assert_eq!(
        Solution::string_matching(vec![
            "blue".to_string(),
            "green".to_string(),
            "bu".to_string()
        ]),
        vec![] as Vec<String>
    );
}

struct Solution;
impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        return words
            .iter()
            .filter(|word| {
                words
                    .iter()
                    .filter(|w| w.len() > word.len())
                    .any(|w| w.contains(*word))
            })
            .map(|w| w.to_string())
            .collect();
    }
}
