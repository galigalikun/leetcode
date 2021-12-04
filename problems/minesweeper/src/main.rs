fn main() {
    assert_eq!(
        Solution::update_board(
            vec![
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'M', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E'],
                vec!['E', 'E', 'E', 'E', 'E']
            ],
            vec![3, 0]
        ),
        vec![
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'M', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ]
    );
    assert_eq!(
        Solution::update_board(
            vec![
                vec!['B', '1', 'E', '1', 'B'],
                vec!['B', '1', 'M', '1', 'B'],
                vec!['B', '1', '1', '1', 'B'],
                vec!['B', 'B', 'B', 'B', 'B']
            ],
            vec![1, 2]
        ),
        vec![
            ['B', '1', 'E', '1', 'B'],
            ['B', '1', 'X', '1', 'B'],
            ['B', '1', '1', '1', 'B'],
            ['B', 'B', 'B', 'B', 'B']
        ]
    );
}

struct Solution {}
// https://xiaoguan.gitbooks.io/leetcode/content/LeetCode/529-minesweeper-medium.html
impl Solution {
    fn helper(board: &mut Vec<Vec<char>>, row: usize, col: usize) {
        let dir = vec![
            vec![-1, -1],
            vec![0, -1],
            vec![1, -1],
            vec![-1, 0],
            vec![1, 0],
            vec![-1, 1],
            vec![0, 1],
            vec![1, 1],
        ];
        let mut mine = 0;
        for i in 0..8 {
            let (x, y) = (row as i32 + dir[i][0], col as i32 + dir[i][1]);
            if x >= 0 && x < board.len() as i32 && y >= 0 && y < board[0].len() as i32 {
                if board[x as usize][y as usize] == 'M' {
                    mine += 1;
                }
            }
        }
        if mine > 0 {
            board[row][col] = std::char::from_digit(mine, 10).unwrap();
        } else {
            board[row][col] = 'B';
            for i in 0..8 {
                let (x, y) = (row as i32 + dir[i][0], col as i32 + dir[i][1]);
                if x >= 0
                    && x < board.len() as i32
                    && y >= 0
                    && y < board[0].len() as i32
                    && board[x as usize][y as usize] == 'E'
                {
                    Solution::helper(board, x as usize, y as usize);
                }
            }
        }
    }
    pub fn update_board(board: Vec<Vec<char>>, click: Vec<i32>) -> Vec<Vec<char>> {
        let mut ans = board.clone();
        let (row, col) = (click[0] as usize, click[1] as usize);
        if board[row][col] == 'M' {
            ans[row][col] = 'X';
            return ans;
        }
        Solution::helper(&mut ans, row, col);

        return ans;
    }
}
