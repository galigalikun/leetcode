use std::collections::{HashMap, HashSet};

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
        let exact_words: HashSet<&str> = wordlist.iter().map(String::as_str).collect();
        let mut case_insensitive_first_match: HashMap<String, &str> = HashMap::new();
        let mut vowel_error_first_match: HashMap<String, &str> = HashMap::new();

        for word in &wordlist {
            let lower = word.to_ascii_lowercase();
            case_insensitive_first_match
                .entry(lower.clone())
                .or_insert(word.as_str());
            vowel_error_first_match
                .entry(Self::devowel(&lower))
                .or_insert(word.as_str());
        }

        let mut answer = Vec::with_capacity(queries.len());
        for query in queries {
            if exact_words.contains(query.as_str()) {
                answer.push(query);
                continue;
            }

            let lower = query.to_ascii_lowercase();
            if let Some(&matched) = case_insensitive_first_match.get(&lower) {
                answer.push(matched.to_string());
                continue;
            }

            if let Some(&matched) = vowel_error_first_match.get(&Self::devowel(&lower)) {
                answer.push(matched.to_string());
                continue;
            }

            answer.push(String::new());
        }

        answer
    }

    fn devowel(word: &str) -> String {
        word.chars()
            .map(|c| match c {
                'a' | 'e' | 'i' | 'o' | 'u' => '*',
                _ => c,
            })
            .collect()
    }
}
