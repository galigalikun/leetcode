fn main() {
    assert_eq!(Solution::reverse_vowels("hello".to_string()), "holle");

    assert_eq!(Solution::reverse_vowels("leetcode".to_string()), "leotcede");

    assert_eq!(Solution::reverse_vowels("aA".to_string()), "Aa");
}

struct Solution {}
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut rev = s
            .chars()
            .filter(|c| match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => true,
                _ => false,
            })
            .rev()
            .collect::<Vec<_>>();
        return s
            .chars()
            .map(|c| match c {
                'A' | 'E' | 'I' | 'O' | 'U' | 'a' | 'e' | 'i' | 'o' | 'u' => rev.remove(0),
                _ => c,
            })
            .collect::<String>();
    }
}
