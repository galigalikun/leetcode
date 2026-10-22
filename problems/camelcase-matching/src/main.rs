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
    fn is_match(query: &str, pattern: &str) -> bool {
        let query_bytes = query.as_bytes();
        let pattern_bytes = pattern.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while i < query_bytes.len() {
            if j < pattern_bytes.len() && query_bytes[i] == pattern_bytes[j] {
                i += 1;
                j += 1;
            } else if query_bytes[i].is_ascii_lowercase() {
                i += 1;
            } else {
                return false;
            }
        }

        j == pattern_bytes.len()
    }
    pub fn camel_match(queries: Vec<String>, pattern: String) -> Vec<bool> {
        let mut result = vec![];
        for query in queries {
            result.push(Solution::is_match(&query, &pattern));
        }
        result
    }
}
