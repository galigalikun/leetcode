fn main() {
    assert_eq!(
        Solution::can_be_equal("abcd".to_string(), "cdab".to_string()),
        true
    );
    assert_eq!(
        Solution::can_be_equal("abcd".to_string(), "dacb".to_string()),
        false
    );
}

struct Solution;
impl Solution {
    pub fn can_be_equal(s1: String, s2: String) -> bool {
        let mut s1 = s1.into_bytes();
        let mut s2 = s2.into_bytes();
        s1.sort_unstable();
        s2.sort_unstable();
        s1 == s2
    }
}
