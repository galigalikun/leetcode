fn main() {
    assert_eq!(Solution::are_occurrences_equal("abacbc".to_string()), true);
    assert_eq!(Solution::are_occurrences_equal("aaabb".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut char_count = std::collections::HashMap::new();
        for c in s.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        let mut occurrences = char_count.values();
        let first = occurrences.next();
        occurrences.all(|&count| count == *first.unwrap())
    }
}
