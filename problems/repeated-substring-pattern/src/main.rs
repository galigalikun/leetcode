fn main() {
    assert_eq!(
        Solution::repeated_substring_pattern("abab".to_string()),
        true
    );
    assert_eq!(
        Solution::repeated_substring_pattern("aba".to_string()),
        false
    );
    assert_eq!(
        Solution::repeated_substring_pattern("abcabcabcabc".to_string()),
        true
    );
    assert_eq!(
        Solution::repeated_substring_pattern("ababab".to_string()),
        true
    );
}

pub struct Solution {}
impl Solution {
    fn helper(s: String, n: usize) -> bool {
        if s.len() < n {
            return false;
        }
        if s.len() % n == 0 {
            let p = &s[0..s.len() / n];
            for i in (s.len() / n..s.len()).step_by(s.len() / n) {
                if p != &s[i..i + s.len() / n] {
                    return Solution::helper(s, n + 1);
                }
            }
            return true;
        }
        return Solution::helper(s, n + 1);
    }
    pub fn repeated_substring_pattern(s: String) -> bool {
        return Solution::helper(s, 2);
    }
}
