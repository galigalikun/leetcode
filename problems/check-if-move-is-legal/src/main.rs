fn main() {
    assert_eq!(
        Solution::check_move(
            vec![
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
                vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', '.', '.', '.', '.']
            ],
            4,
            3,
            'B'
        ),
        true
    );
    assert_eq!(
        Solution::check_move(
            vec![
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', 'B', '.', '.', 'W', '.', '.', '.'],
                vec!['.', '.', 'W', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', 'W', 'B', '.', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['.', '.', '.', '.', 'B', 'W', '.', '.'],
                vec!['.', '.', '.', '.', '.', '.', 'W', '.'],
                vec!['.', '.', '.', '.', '.', '.', '.', 'B']
            ],
            4,
            4,
            'W'
        ),
        false
    );
}

struct Solution;
impl Solution {
    pub fn check_move(board: Vec<Vec<char>>, r_move: i32, c_move: i32, color: char) -> bool {
        let opponent = if color == 'W' { 'B' } else { 'W' };
        let directions = vec![(1, 0), (0, 1), (1, 1), (1, -1)];
        for (dr, dc) in directions {
            let mut r = r_move;
            let mut c = c_move;
            let mut found_opponent = false;
            while r >= 0 && r < 8 && c >= 0 && c < 8 {
                match board[r as usize][c as usize] {
                    '.' => {
                        if found_opponent {
                            return true;
                        }
                    }
                    _ if board[r as usize][c as usize] == opponent => {
                        found_opponent = true;
                    }
                    _ => {
                        if found_opponent {
                            return true;
                        }
                    }
                }
                r += dr;
                c += dc;
            }
        }
        return false;
    }
}
