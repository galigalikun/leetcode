fn main() {
    assert_eq!(
        Solution::spellchecker(
            vec![
                "KiTe".to_string(),
                "kite".to_string(),
                "hare".to_string(),
                "Hare".to_string()
            ],
            vec![
                "kite".to_string(),
                "Kite".to_string(),
                "KiTe".to_string(),
                "Hare".to_string(),
                "HARE".to_string(),
                "Hear".to_string(),
                "hear".to_string(),
                "keti".to_string(),
                "keet".to_string(),
                "keto".to_string()
            ]
        ),
        vec!["kite", "KiTe", "KiTe", "Hare", "hare", "", "", "KiTe", "", "KiTe"]
    );
    assert_eq!(
        Solution::spellchecker(vec!["yellow".to_string()], vec!["YellOw".to_string()]),
        vec!["yellow"]
    );
}

struct Solution;
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        return vec![];
    }
}
