fn main() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1]
        ]),
        2
    );
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]),
        0
    );
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]),
        -1
    );
}

struct Solution {}
impl Solution {
    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let n = board.len();

        for i in 0..n {
            for j in 0..n {
                if (board[0][0] ^ board[i][0] ^ board[0][j] ^ board[i][j]) != 0 {
                    return -1;
                }
            }
        }

        let row_sum: usize = (0..n).map(|i| board[0][i] as usize).sum();
        let col_sum: usize = (0..n).map(|i| board[i][0] as usize).sum();

        if row_sum < n / 2 || row_sum > (n + 1) / 2 {
            return -1;
        }
        if col_sum < n / 2 || col_sum > (n + 1) / 2 {
            return -1;
        }

        let mut row_swaps = 0usize;
        let mut col_swaps = 0usize;

        for i in 0..n {
            if board[i][0] == (i % 2) as i32 {
                row_swaps += 1;
            }
            if board[0][i] == (i % 2) as i32 {
                col_swaps += 1;
            }
        }

        if n % 2 == 1 {
            if row_swaps % 2 == 1 {
                row_swaps = n - row_swaps;
            }
            if col_swaps % 2 == 1 {
                col_swaps = n - col_swaps;
            }
        } else {
            row_swaps = row_swaps.min(n - row_swaps);
            col_swaps = col_swaps.min(n - col_swaps);
        }

        ((row_swaps + col_swaps) / 2) as i32
    }
}
