fn main() {
    assert_eq!(
        Solution::k_similarity("ab".to_string(), "ba".to_string()),
        1
    );
    assert_eq!(
        Solution::k_similarity("abc".to_string(), "bca".to_string()),
        2
    );
}

struct Solution;
impl Solution {
    pub fn k_similarity(s1: String, s2: String) -> i32 {
        return 0;
    }
}
