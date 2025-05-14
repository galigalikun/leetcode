fn main() {
    assert_eq!(
        Solution::longest_beautiful_substring("aeiaaioaaaaeiiiiouuuooaauuaeiu".to_string()),
        13
    );
    assert_eq!(
        Solution::longest_beautiful_substring("aeeeiiiioooauuuaeiou".to_string()),
        13
    );
    assert_eq!(Solution::longest_beautiful_substring("a".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn longest_beautiful_substring(word: String) -> i32 {
        let mut max_length = 0;
        let mut current_length = 0;
        let mut last_char = ' ';
        let mut vowel_count = 0;
        let mut vowel_set = vec![false; 5];
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        for c in word.chars() {
            if c == last_char {
                current_length += 1;
            } else {
                if last_char != ' ' {
                    if vowel_count == 5 {
                        max_length = max_length.max(current_length);
                    }
                    current_length = 1;
                }
                last_char = c;
                if let Some(index) = vowels.iter().position(|&v| v == c) {
                    if !vowel_set[index] {
                        vowel_set[index] = true;
                        vowel_count += 1;
                    }
                }
            }
        }
        if vowel_count == 5 {
            max_length = max_length.max(current_length);
        }
        return max_length as i32;
    }
}
