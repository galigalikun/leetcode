fn main() {
    assert_eq!(
        Solution::check_if_pangram("thequickbrownfoxjumpsoverthelazydog".to_string()),
        true
    );
    assert_eq!(Solution::check_if_pangram("leetcode".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut seen = [false; 26];
        for c in sentence.chars() {
            if c.is_ascii_alphabetic() {
                seen[c.to_ascii_lowercase() as usize - 'a' as usize] = true;
            }
        }
        seen.iter().all(|&x| x)
    }
}
