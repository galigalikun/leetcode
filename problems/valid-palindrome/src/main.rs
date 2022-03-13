fn main() {
    assert_eq!(
        Solution::is_palindrome("A man, a plan, a canal: Panama".to_string()),
        true
    );
    assert_eq!(Solution::is_palindrome("race a car".to_string()), false);
    assert_eq!(Solution::is_palindrome("0P".to_string()), false);
}

struct Solution {}
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let ss = s
            .as_str()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>();

        if ss == ss.as_str().chars().rev().collect::<String>() {
            return true;
        }

        return false;
    }
}
