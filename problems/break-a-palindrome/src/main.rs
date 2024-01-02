fn main() {
    assert_eq!(
        Solution::break_palindrome(String::from("abccba")),
        String::from("aaccba")
    );
    assert_eq!(
        Solution::break_palindrome(String::from("a")),
        String::from("")
    );
    assert_eq!(
        Solution::break_palindrome(String::from("aa")),
        String::from("ab")
    );
    assert_eq!(
        Solution::break_palindrome(String::from("aba")),
        String::from("abb")
    );
    assert_eq!(
        Solution::break_palindrome(String::from("aaa")),
        String::from("aab")
    );
    assert_eq!(
        Solution::break_palindrome(String::from("aaaa")),
        String::from("aaab")
    );
    assert_eq!(
        Solution::break_palindrome(String::from("aaaaa")),
        String::from("aaaab")
    );
}

struct Solution;
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let mut palindrome = palindrome.into_bytes();
        let len = palindrome.len();
        if len == 1 {
            return String::from("");
        }
        let mut i = 0;
        while i < len / 2 {
            if palindrome[i] != b'a' {
                palindrome[i] = b'a';
                return String::from_utf8(palindrome).unwrap();
            }
            i += 1;
        }
        palindrome[len - 1] = b'b';
        return String::from_utf8(palindrome).unwrap();
    }
}
