fn main() {
    assert_eq!(Solution::ambiguous_coordinates("(123)".to_string()), vec!["(1, 2.3)","(1, 23)","(1.2, 3)","(12, 3)"]);
    assert_eq!(Solution::ambiguous_coordinates("(0123)".to_string()), vec!["(0, 1.23)","(0, 12.3)","(0, 123)","(0.1, 2.3)","(0.1, 23)","(0.12, 3)"]);
    assert_eq!(Solution::ambiguous_coordinates("(00011)".to_string()), vec!["(0, 0.011)","(0.001, 1)"]);
}

struct Solution{}
impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        return vec![];
    }
}
