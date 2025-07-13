fn main() {
    assert_eq!(
        Solution::remove_occurrences("daabcbaabcbc".to_string(), "abc".to_string()),
        "dab".to_string()
    );
    assert_eq!(
        Solution::remove_occurrences("axxxxyyyyb".to_string(), "xy".to_string()),
        "ab".to_string()
    );
}

struct Solution;
impl Solution {
    fn remove(s: String, part: &str) -> String {
        let mut result = s;
        while let Some(pos) = result.find(part) {
            result.replace_range(pos..pos + part.len(), "");
        }
        result
    }
    pub fn remove_occurrences(s: String, part: String) -> String {
        return Self::remove(s, &part);
    }
}
