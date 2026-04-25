fn main() {
    assert_eq!(
        Solution::find_ladders(
            "hit".to_string(),
            "cog".to_string(),
            vec![
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        ),
        vec![
            [
                "hit".to_string(),
                "hot".to_string(),
                "dot".to_string(),
                "dog".to_string(),
                "cog".to_string()
            ],
            [
                "hit".to_string(),
                "hot".to_string(),
                "lot".to_string(),
                "log".to_string(),
                "cog".to_string()
            ]
        ]
    );
}

struct Solution {}

use std::collections::{BTreeSet, HashMap, HashSet};

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut dictionary: HashSet<String> = word_list.into_iter().collect();
        if !dictionary.contains(&end_word) {
            return vec![];
        }
        dictionary.insert(begin_word.clone());

        let mut parents: HashMap<String, Vec<String>> = HashMap::new();
        let mut visited: HashSet<String> = HashSet::from([begin_word.clone()]);
        let mut current_level = vec![begin_word.clone()];
        let mut found = false;

        while !current_level.is_empty() && !found {
            let mut next_level: BTreeSet<String> = BTreeSet::new();
            let mut next_seen: HashSet<String> = HashSet::new();

            for word in &current_level {
                let mut chars: Vec<char> = word.chars().collect();
                for i in 0..chars.len() {
                    let original = chars[i];
                    for b in b'a'..=b'z' {
                        let candidate_char = b as char;
                        if candidate_char == original {
                            continue;
                        }
                        chars[i] = candidate_char;
                        let candidate: String = chars.iter().collect();

                        if !dictionary.contains(&candidate) || visited.contains(&candidate) {
                            continue;
                        }

                        parents
                            .entry(candidate.clone())
                            .or_default()
                            .push(word.clone());

                        if candidate == end_word {
                            found = true;
                        }
                        if next_seen.insert(candidate.clone()) {
                            next_level.insert(candidate);
                        }
                    }
                    chars[i] = original;
                }
            }

            visited.extend(next_seen);
            current_level = next_level.into_iter().collect();
        }

        if !found {
            return vec![];
        }

        let mut result = vec![];
        let mut path = vec![end_word.clone()];
        Self::build_paths(&end_word, &begin_word, &parents, &mut path, &mut result);
        result
    }

    fn build_paths(
        current: &str,
        begin_word: &str,
        parents: &HashMap<String, Vec<String>>,
        path: &mut Vec<String>,
        result: &mut Vec<Vec<String>>,
    ) {
        if current == begin_word {
            let mut built = path.clone();
            built.reverse();
            result.push(built);
            return;
        }

        if let Some(prev_words) = parents.get(current) {
            for prev in prev_words {
                path.push(prev.clone());
                Self::build_paths(prev, begin_word, parents, path, result);
                path.pop();
            }
        }
    }
}
