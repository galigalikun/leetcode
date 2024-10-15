fn main() {
    assert_eq!(
        Solution::check_palindrome_formation("x".to_string(), "y".to_string()),
        true
    );
    assert_eq!(
        Solution::check_palindrome_formation("xbdef".to_string(), "xecab".to_string()),
        false
    );
    assert_eq!(
        Solution::check_palindrome_formation("ulacfd".to_string(), "jizalu".to_string()),
        true
    );
}

struct Solution;
impl Solution {
    fn check_palindrome_formation_helper(a: String, b: String) -> bool {
        let mut i = 0;
        let mut j = a.len() - 1;
        let a = a.as_bytes();
        let b = b.as_bytes();
        while i < j {
            if a[i] != b[j] {
                break;
            }
            i += 1;
            j -= 1;
        }
        while i < j {
            if a[j] != b[i] {
                break;
            }
            i += 1;
            j -= 1;
        }
        return i >= j;
    }
    pub fn check_palindrome_formation(a: String, b: String) -> bool {
        return Self::check_palindrome_formation_helper(a.clone(), b.clone())
            || Self::check_palindrome_formation_helper(b, a);
    }
}
