fn main() {
    assert_eq!(
        Solution::num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'R', '.', '.', '.', 'p'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
    assert_eq!(
        Solution::num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'B', 'R', 'B', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'B', 'p', 'p', '.', '.'],
            vec!['.', 'p', 'p', 'p', 'p', 'p', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        0
    );
    assert_eq!(
        Solution::num_rook_captures(vec![
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['p', 'p', '.', 'R', '.', 'p', 'B', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
            vec!['.', '.', '.', 'p', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.']
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    x = i;
                    y = j;
                    break;
                }
            }
        }
        let mut count = 0;
        for i in (0..x).rev() {
            if board[i][y] == 'p' {
                count += 1;
                break;
            } else if board[i][y] == 'B' {
                break;
            }
        }
        for i in x + 1..8 {
            if board[i][y] == 'p' {
                count += 1;
                break;
            } else if board[i][y] == 'B' {
                break;
            }
        }
        for i in (0..y).rev() {
            if board[x][i] == 'p' {
                count += 1;
                break;
            } else if board[x][i] == 'B' {
                break;
            }
        }
        for i in y + 1..8 {
            if board[x][i] == 'p' {
                count += 1;
                break;
            } else if board[x][i] == 'B' {
                break;
            }
        }
        count
    }
}
