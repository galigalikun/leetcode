fn main() {
    assert_eq!(
        Solution::max_length_between_equal_characters("aa".to_string()),
        0
    );
    assert_eq!(
        Solution::max_length_between_equal_characters("abca".to_string()),
        2
    );
    assert_eq!(
        Solution::max_length_between_equal_characters("cbzxy".to_string()),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut max = -1;
        let mut map = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(j) = map.get(&c) {
                max = max.max((i - j - 1) as i32);
            } else {
                map.insert(c, i);
            }
        }
        max
    }
}
