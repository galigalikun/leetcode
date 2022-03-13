fn main() {
    let mut board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, vec![[0, 0, 0], [1, 0, 1], [0, 1, 1], [0, 1, 0]]);

    board = vec![vec![1, 1], vec![1, 0]];
    Solution::game_of_life(&mut board);
    assert_eq!(board, vec![[1, 1], [1, 1]]);
}

struct Solution {}
// https://cheonhyangzhang.gitbooks.io/leetcode-solutions/content/289-game-of-life.html
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        for x in 0 as i32..board.len() as i32 {
            for y in 0 as i32..board[x as usize].len() as i32 {
                let mut num = 0;
                for i in x - 1..x + 2 {
                    for j in y - 1..y + 2 {
                        if !(i == x && j == y) {
                            if i < 0
                                || j < 0
                                || i >= board.len() as i32
                                || j >= board[x as usize].len() as i32
                            {
                            } else if board[i as usize][j as usize] >= 1 {
                                num += 1;
                            }
                        }
                    }
                }
                if board[x as usize][y as usize] >= 1 {
                    if num < 2 || num > 3 {
                        board[x as usize][y as usize] = 2;
                    }
                } else {
                    if num == 3 {
                        board[x as usize][y as usize] = -1;
                    }
                }
            }
        }
        for x in 0..board.len() {
            for y in 0..board[x].len() {
                if board[x][y] == 2 {
                    board[x][y] = 0;
                } else if board[x][y] == -1 {
                    board[x][y] = 1;
                }
            }
        }
    }
}
