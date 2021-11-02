fn main() {
    assert_eq!(
        Solution::count_battleships(vec![
            vec!['X', '.', '.', 'X'],
            vec!['.', '.', '.', 'X'],
            vec!['.', '.', '.', 'X']
        ]),
        2
    );
    assert_eq!(Solution::count_battleships(vec![vec!['.']]), 0);
    assert_eq!(
        Solution::count_battleships(vec![vec!['.', '.'], vec!['X', 'X']]),
        1
    );
}

pub struct Solution {}
impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut result = 0;
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == 'X' {
                    if y == 0 {
                        if x == 0 {
                            result += 1;
                        } else {
                            if board[y][x - 1] != 'X' {
                                result += 1;
                            }
                        }
                    } else {
                        if x == 0 {
                            if board[y - 1][x] != 'X' {
                                result += 1;
                            }
                        } else {
                            if board[y - 1][x] != 'X' && board[y][x - 1] != 'X' {
                                result += 1;
                            }
                        }
                    }
                }
            }
        }
        return result;
    }
}
