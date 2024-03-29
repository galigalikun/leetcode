fn main() {
    assert_eq!(
        Solution::reverse_words("the sky is blue".to_string()),
        "blue is sky the".to_string()
    );
    assert_eq!(
        Solution::reverse_words("  hello world  ".to_string()),
        "world hello".to_string()
    );
    assert_eq!(
        Solution::reverse_words("a good   example".to_string()),
        "example good a".to_string()
    );
    assert_eq!(
        Solution::reverse_words("  Bob    Loves  Alice   ".to_string()),
        "Alice Loves Bob".to_string()
    );
    assert_eq!(
        Solution::reverse_words("Alice does not even like bob".to_string()),
        "bob like even not does Alice".to_string()
    );
}

struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut ss = s.trim().split_whitespace().collect::<Vec<_>>();
        ss.reverse();
        return ss.join(" ");
    }
}
