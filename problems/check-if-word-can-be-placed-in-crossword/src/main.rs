fn main() {
    assert_eq!(
        Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec![' ', ' ', '#'],
                vec!['#', 'c', ' ']
            ],
            "abc".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::place_word_in_crossword(
            vec![
                vec![' ', '#', 'a'],
                vec![' ', '#', 'c'],
                vec![' ', '#', 'a']
            ],
            "ac".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec![' ', ' ', ' '],
                vec!['#', ' ', 'c']
            ],
            "ca".to_string()
        ),
        true
    );
}

struct Solution;
impl Solution {
    fn transpose(board: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let rows = board.len();
        let cols = board[0].len();
        let mut transposed = vec![vec![' '; rows]; cols];
        for r in 0..rows {
            for c in 0..cols {
                transposed[c][r] = board[r][c];
            }
        }
        transposed
    }
    fn check_board(board: &Vec<Vec<char>>, word: &String) -> bool {
        let word_chars: Vec<char> = word.chars().collect();
        let word_len = word_chars.len();

        for row in board {
            let mut count = 0;
            for &cell in row {
                if cell == ' ' || cell == word_chars[count] {
                    count += 1;
                    if count == word_len {
                        return true;
                    }
                } else {
                    count = 0;
                }
            }
        }
        false
    }
    pub fn place_word_in_crossword(board: Vec<Vec<char>>, word: String) -> bool {
        return Self::check_board(&board, &word)
            || Self::check_board(&Self::transpose(&board), &word);
    }
}
