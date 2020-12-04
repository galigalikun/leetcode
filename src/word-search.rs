fn main() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "word: String".to_string()
        ),
        true
    );
}

pub struct Solution {}
impl Solution {
    fn check(board: Vec<Vec<char>>, x: usize, y: usize) -> bool {

        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut idx = 0;
        let mut chars = word.as_str().chars();
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if let Some(c) = chars.nth(idx) {
                    if c == board[y][x] {
                        idx+=1;
                    }
                }
            }
        }
        return false;
    }
}
