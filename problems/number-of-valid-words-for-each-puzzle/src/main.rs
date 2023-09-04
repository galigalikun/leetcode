fn main() {
    assert_eq!(
        Solution::find_num_of_valid_words(
            vec![
                "aaaa".to_string(),
                "asas".to_string(),
                "able".to_string(),
                "ability".to_string(),
                "actt".to_string(),
                "actor".to_string(),
                "access".to_string()
            ],
            vec![
                "aboveyz".to_string(),
                "abrodyz".to_string(),
                "abslute".to_string(),
                "absoryz".to_string(),
                "actresz".to_string(),
                "gaswxyz".to_string()
            ]
        ),
        vec![1, 1, 3, 2, 4, 0]
    );
    assert_eq!(
        Solution::find_num_of_valid_words(
            vec![
                "apple".to_string(),
                "pleas".to_string(),
                "please".to_string()
            ],
            vec![
                "aelwxyz".to_string(),
                "aelpxyz".to_string(),
                "aelpsxy".to_string(),
                "saelpxy".to_string(),
                "xaelpsy".to_string()
            ]
        ),
        vec![0, 1, 3, 2, 0]
    );
}

struct Solution;
impl Solution {
    fn is_valid(puzzle: &String, word: &String) -> bool {
        let puzzle_chars: Vec<char> = puzzle.chars().collect();
        let word_chars: Vec<char> = word.chars().collect();
        let first_char = puzzle_chars[0];
        if !word_chars.contains(&first_char) {
            return false;
        }
        for word_char in &word_chars {
            if !puzzle_chars.contains(word_char) {
                return false;
            }
        }
        return true;
    }
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        let mut result = vec![];
        for puzzle in puzzles {
            let mut count = 0;
            for word in &words {
                if Self::is_valid(&puzzle, &word) {
                    count += 1;
                }
            }
            result.push(count);
        }
        return result;
    }
}
