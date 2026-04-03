fn main() {
    assert_eq!(
        Solution::check_strings("abcdba".to_string(), "cabdab".to_string()),
        true
    );
    assert_eq!(
        Solution::check_strings("abe".to_string(), "bea".to_string()),
        false
    );
}

struct Solution;
impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        s1.chars()
            .zip(s2.chars())
            .all(|(c1, c2)| c1 == c2 || c1 == '?' || c2 == '?')
    }
}
