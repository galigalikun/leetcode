fn main() {
    assert_eq!(Solution::buddy_strings("ab".to_string(), "ba".to_string()), true);
    assert_eq!(Solution::buddy_strings("ab".to_string(), "ab".to_string()), false);
    assert_eq!(Solution::buddy_strings("aa".to_string(), "aa".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        return false;
    }
}
