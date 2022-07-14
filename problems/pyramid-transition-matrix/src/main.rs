fn main() {
    assert_eq!(
        Solution::pyramid_transition(
            "BCD".to_string(),
            vec![
                "BCC".to_string(),
                "CDE".to_string(),
                "CEA".to_string(),
                "FFF".to_string()
            ]
        ),
        true
    );
    assert_eq!(
        Solution::pyramid_transition(
            "AAAA".to_string(),
            vec![
                "AAB".to_string(),
                "AAC".to_string(),
                "BCD".to_string(),
                "BBE".to_string(),
                "DEF".to_string()
            ]
        ),
        false
    );
}

struct Solution {}
impl Solution {
    pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
        return false;
    }
}
