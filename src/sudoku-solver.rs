fn main() {
    let board = &mut vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(board);
    assert_eq!(
        board,
        &mut vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9']
        ]
    );
}

pub struct Solution {}
impl Solution {
    fn checker(
        board: &mut Vec<Vec<char>>,
        validate: &mut Vec<Vec<Vec<char>>>,
        xx: usize,
        yy: usize,
    ) {
        // 0 1 2 -> /3 -> 0 0 0
        // 3 4 5 -> /3 -> 1 1 1
        // 6 2 8 4 1 9 5 7 9
        for x in 0..board.len() {
            for y in 0..board[x].len() {
                if board[x][y] != '.' {
                    if x == xx {
                        validate[xx][yy].retain(|&c| c != board[x][y]);
                    } else if y == yy {
                        validate[xx][yy].retain(|&c| c != board[x][y]);
                    } else if x / 3 == xx / 3 && y / 3 == yy / 3 {
                        validate[xx][yy].retain(|&c| c != board[x][y]);
                    }
                }
            }
        }
        if validate[xx][yy].len() == 1 {
            board[xx][yy] = validate[xx][yy][0];
            validate[xx][yy] = vec![];
            // Solution::revival(board[xx][yy], board, validate, xx, yy);
        }
    }
    fn revival(
        ch: char,
        board: &mut Vec<Vec<char>>,
        validate: &mut Vec<Vec<Vec<char>>>,
        xx: usize,
        yy: usize,
    ) {
        for y in (0..yy).rev() {
            validate[xx][y].retain(|&c| c != ch);
            if validate[xx][y].len() == 1 {
                board[xx][y] = validate[xx][y][0];
                validate[xx][y] = vec![];
                Solution::revival(board[xx][y], board, validate, xx, y);
            }
        }
        for x in (0..xx).rev() {
            validate[x][yy].retain(|&c| c != ch);
            if validate[x][yy].len() == 1 {
                board[x][yy] = validate[x][yy][0];
                validate[x][yy] = vec![];
                Solution::revival(board[x][yy], board, validate, x, yy);
            }
        }
        for x in (xx / 3) * 3..((xx + 1) / 3) * 3 {
            for y in (yy / 3) * 3..yy {
                validate[x][y].retain(|&c| c != ch);
                if validate[x][y].len() == 1 {
                    board[x][y] = validate[x][y][0];
                    validate[x][y] = vec![];
                    Solution::revival(board[x][y], board, validate, x, y);
                }
            }
        }
    }
    fn dfs(
        mut x: usize,
        mut y: usize,
        board: &mut Vec<Vec<char>>,
        rows: &mut Vec<u64>,
        cols: &mut Vec<u64>,
        squares: &mut Vec<u64>,
    ) -> bool {
        if y == 9 {
            x += 1;
            y = 0;
        }
        if x == 9 {
            return true;
        }

        if board[x][y] == '.' {
            for num in 0..9 {
                if !((rows[x] & 1 << num) != 0
                    || (cols[y] & 1 << num) != 0
                    || (squares[3 * (x / 3) + y / 3] & 1 << num) != 0)
                {
                    rows[x] |= 1 << num;
                    cols[y] |= 1 << num;
                    squares[3 * (x / 3) + y / 3] |= 1 << num;
                    if let Some(c) = std::char::from_digit(num+1, 10) {
                        board[x][y] = c;
                    }

                    if Solution::dfs(x, y + 1, board, rows, cols, squares) {
                        return true;
                    }
                    rows[x] -= 1 << num;
                    cols[y] -= 1 << num;
                    squares[3 * (x / 3) + y / 3] -= 1 << num;
                    board[x][y] = '.';
                }
            }
        } else {
            if Solution::dfs(x, y + 1, board, rows, cols, squares) {
                return true;
            }
        }
        return false;
    }
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let rows:&mut Vec<u64> = &mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let cols:&mut Vec<u64> = &mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        let squares:&mut Vec<u64> = &mut vec![0, 0, 0, 0, 0, 0, 0, 0, 0];
        for x in 0..board.len() {
            for y in 0..board[x].len() {
                if let Some(num) = board[x][y].to_digit(10) {
                    let n = num - 1;
                    rows[x] |= 1 << n;
                    cols[y] |= 1 << n;
                    squares[3 * (x / 3) + y / 3] |= 1 << n;
                }

            }
        }
        Solution::dfs(0, 0, board, rows, cols, squares);
    }
}
