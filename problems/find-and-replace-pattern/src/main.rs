fn main() {
    assert_eq!(Solution::find_and_replace_pattern(vec!["abc".to_string(),"deq".to_string(),"mee".to_string(),"aqq".to_string(),"dkd".to_string(),"ccc".to_string()], "abb".to_string()), vec!["mee","aqq"]);
    assert_eq!(Solution::find_and_replace_pattern(vec!["a".to_string(),"b".to_string(),"c".to_string()], "a".to_string()), vec!["a","b","c"]);
}

struct Solution;
impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        fn normalize(s: &str) -> Vec<usize> {
            let mut first_seen = std::collections::HashMap::new();
            s.chars()
                .map(|c| {
                    let next = first_seen.len();
                    *first_seen.entry(c).or_insert(next)
                })
                .collect()
        }

        let target = normalize(&pattern);
        words
            .into_iter()
            .filter(|w| normalize(w) == target)
            .collect()
    }
}
