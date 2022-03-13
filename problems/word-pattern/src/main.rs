fn main() {
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat dog".to_string()),
        true
    );
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog cat cat fish".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("abba".to_string(), "dog dog dog dog".to_string()),
        false
    );
    assert_eq!(
        Solution::word_pattern("jquery".to_string(), "jquery".to_string()),
        false
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if s.split_whitespace().collect::<Vec<_>>().len() != pattern.len() {
            return false;
        }
        let mut i = 0;
        let mut map: HashMap<Option<char>, &str> = HashMap::new();
        for c in s.split_whitespace() {
            if let Some(&key) = map.get(&pattern.chars().nth(i)) {
                if key != c {
                    return false;
                }
            } else {
                for val in map.values() {
                    if val == &c {
                        return false;
                    }
                }
                map.insert(pattern.chars().nth(i), c);
            }
            i += 1;
        }
        return true;
    }
}
