fn main() {
    assert_eq!(
        Solution::camel_match(
            vec![
                "FooBar".to_string(),
                "FooBarTest".to_string(),
                "FootBall".to_string(),
                "FrameBuffer".to_string(),
                "ForceFeedBack".to_string()
            ],
            "FB".to_string()
        ),
        vec![true, false, true, true, false]
    );
    assert_eq!(
        Solution::camel_match(
            vec![
                "FooBar".to_string(),
                "FooBarTest".to_string(),
                "FootBall".to_string(),
                "FrameBuffer".to_string(),
                "ForceFeedBack".to_string()
            ],
            "FoBa".to_string()
        ),
        vec![true, false, true, false, false]
    );
    assert_eq!(
        Solution::camel_match(
            vec![
                "FooBar".to_string(),
                "FooBarTest".to_string(),
                "FootBall".to_string(),
                "FrameBuffer".to_string(),
                "ForceFeedBack".to_string()
            ],
            "FoBaT".to_string()
        ),
        vec![false, true, false, false, false]
    );
}

struct Solution;
impl Solution {
    fn is_match(query: String, pattern: String) -> bool {
        let mut query_chars = query.chars();
        let mut pattern_chars = pattern.chars();
        loop {
            let query_char = query_chars.next();
            let pattern_char = pattern_chars.next();
            if query_char.is_none() {
                return pattern_char.is_none();
            }
            if pattern_char.is_none() {
                return query_char.unwrap().is_lowercase();
            }
            if query_char.unwrap() == pattern_char.unwrap() {
                continue;
            }
            if query_char.unwrap().is_lowercase() {
                continue;
            }
            return false;
        }
    }
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = vec![];
        for query in queries {
            result.push(Solution::is_match(query, pattern.clone()));
        }
        return result;
    }
}
