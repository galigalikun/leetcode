fn main() {
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidbaooo".to_string()),
        true
    );
    assert_eq!(
        Solution::check_inclusion("ab".to_string(), "eidboaoo".to_string()),
        false
    );
    assert_eq!(
        Solution::check_inclusion("abc".to_string(), "bbbca".to_string()),
        true
    );
}

struct Solution {}
impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.contains(&s1) {
            return true;
        }
        if s2.contains(&s1.chars().rev().collect::<String>()) {
            return true;
        }
        return false;
    }
}
