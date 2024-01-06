fn main() {
    assert_eq!(Solution::remove_palindrome_sub("ababa".to_string()), 1);
    assert_eq!(Solution::remove_palindrome_sub("abb".to_string()), 2);
    assert_eq!(Solution::remove_palindrome_sub("baabb".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn remove_palindrome_sub(s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut i = 0;
        let mut j = s.len() - 1;
        for _ in 0..s.len() {
            if s.chars().nth(i).unwrap() != s.chars().nth(j).unwrap() {
                return 2;
            }
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        return 1;
    }
}
