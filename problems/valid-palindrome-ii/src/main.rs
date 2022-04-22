fn main() {
    assert_eq!(Solution::valid_palindrome("aba".to_string()), true);
    assert_eq!(Solution::valid_palindrome("abca".to_string()), true);
    assert_eq!(Solution::valid_palindrome("abc".to_string()), false);
}

struct Solution{}
impl Solution {
    pub fn valid_palindrome(s: String) -> bool {
        return false;
    }
}
