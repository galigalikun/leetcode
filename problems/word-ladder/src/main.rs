fn main() {
    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec!["hot", "dot", "dog", "lot", "log", "cog"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        5
    );
    assert_eq!(
        Solution::ladder_length(
            "hit".to_string(),
            "cog".to_string(),
            vec!["hot", "dot", "dog", "lot", "log"]
                .iter()
                .map(|s| s.to_string())
                .collect()
        ),
        0
    );
}

struct Solution;
impl Solution {
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_list = word_list;
        if !word_list.contains(&end_word) {
            return 0;
        }
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((begin_word, 1));
        while let Some((word, length)) = queue.pop_front() {
            if word == end_word {
                return length;
            }
            for i in 0..word.len() {
                let mut chars: Vec<char> = word.chars().collect();
                for c in 'a'..='z' {
                    chars[i] = c;
                    let new_word: String = chars.iter().collect();
                    if word_list.contains(&new_word) {
                        queue.push_back((new_word.clone(), length + 1));
                        word_list.retain(|w| w != &new_word);
                    }
                }
            }
        }
        0
    }
}
