fn main() {
    assert_eq!(Solution::is_alien_sorted(vec!["hello".to_string(),"leetcode".to_string()], "hlabcdefgijkmnopqrstuvwxyz".to_string()), true);
    assert_eq!(Solution::is_alien_sorted(vec!["word".to_string(),"world".to_string(),"row".to_string()], "worldabcefghijkmnpqstuvxyz".to_string()), false);
    assert_eq!(Solution::is_alien_sorted(vec!["apple".to_string(),"app".to_string()], "abcdefghijklmnopqrstuvwxyz".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        return false;       
    }
}
