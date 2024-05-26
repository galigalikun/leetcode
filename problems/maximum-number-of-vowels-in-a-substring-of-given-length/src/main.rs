fn main() {
    assert_eq!(Solution::max_vowels("abciiidef".to_string(), 3), 3);
    assert_eq!(Solution::max_vowels("aeiou".to_string(), 2), 2);
    assert_eq!(Solution::max_vowels("leetcode".to_string(), 3), 2);
}

struct Solution;
impl Solution {
    fn is_vowel(c: u8) -> bool {
        match c {
            b'a' | b'e' | b'i' | b'o' | b'u' => true,
            _ => false,
        }
    }
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let mut max = 0;
        let mut count = 0;
        let k = k as usize;
        let s = s.as_bytes();
        for i in 0..s.len() {
            if i >= k {
                if Self::is_vowel(s[i - k]) {
                    count -= 1;
                }
            }
            if Self::is_vowel(s[i]) {
                count += 1;
            }
            max = max.max(count);
        }
        return max as i32;
    }
}
