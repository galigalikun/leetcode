fn main() {
    assert_eq!(Solution::halves_are_alike("book".to_string()), true);
    assert_eq!(Solution::halves_are_alike("textbook".to_string()), false);
}

struct Solution;
impl Solution {
    fn count_vowels(s: &str) -> i32 {
        let mut count = 0;
        for c in s.chars() {
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => count += 1,
                _ => {}
            }
        }
        return count;
    }
    pub fn halves_are_alike(s: String) -> bool {
        return Self::count_vowels(&s[..s.len() / 2]) == Self::count_vowels(&s[s.len() / 2..]);
    }
}
