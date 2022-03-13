fn main() {
    {
        let mut board = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];
        Solution::solve(&mut board);
        assert_eq!(
            board,
            [
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X'],
                ['X', 'O', 'X', 'X']
            ]
        );
    }
    {
        let mut board = vec![vec!['X']];
        Solution::solve(&mut board);
        assert_eq!(board, [['X']]);
    }
    {
        let mut board = vec![vec!['O']];
        Solution::solve(&mut board);
        assert_eq!(board, [['O']]);
    }
    {
        let mut board = vec![vec!['O', 'O'], vec!['O', 'O']];
        Solution::solve(&mut board);
        assert_eq!(board, [['O', 'O'], ['O', 'O']]);
    }
}

// https://programmerstart.com/article/99921031248/
struct Solution {}
impl Solution {
    fn helper(board: &mut Vec<Vec<char>>, y: usize, x: usize) {
        board[y][x] = '#';
        if y >= 1 && board[y - 1][x] == 'O' {
            Solution::helper(board, y - 1, x);
        }
        if y + 1 < board.len() && board[y + 1][x] == 'O' {
            Solution::helper(board, y + 1, x);
        }
        if x >= 1 && board[y][x - 1] == 'O' {
            Solution::helper(board, y, x - 1);
        }
        if x + 1 < board[y].len() && board[y][x + 1] == 'O' {
            Solution::helper(board, y, x + 1);
        }
    }
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.len() <= 1 {
            return;
        }
        for x in 0..board[0].len() {
            if board[0][x] == 'O' {
                Solution::helper(board, 0, x);
            }

            if board[board.len() - 1][x] == 'O' {
                Solution::helper(board, board.len() - 1, x);
            }
        }
        for y in 0..board.len() {
            if board[y][0] == 'O' {
                Solution::helper(board, y, 0);
            }

            if board[y][board[y].len() - 1] == 'O' {
                Solution::helper(board, y, board[y].len() - 1);
            }
        }

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == 'O' {
                    board[y][x] = 'X';
                }
            }
        }

        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if board[y][x] == '#' {
                    board[y][x] = 'O';
                }
            }
        }
    }
}
