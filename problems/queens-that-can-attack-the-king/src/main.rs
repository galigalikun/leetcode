fn main() {
    assert_eq!(
        Solution::queens_attackthe_king(
            vec![
                vec![0, 1],
                vec![1, 0],
                vec![4, 0],
                vec![0, 4],
                vec![3, 3],
                vec![2, 4]
            ],
            vec![0, 0]
        ),
        vec![[0, 1], [1, 0], [3, 3]]
    );
    assert_eq!(
        Solution::queens_attackthe_king(
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 4],
                vec![3, 5],
                vec![4, 4],
                vec![4, 5]
            ],
            vec![3, 3]
        ),
        vec![[2, 2], [3, 4], [4, 4]]
    );
}

struct Solution;
impl Solution {
    pub fn queens_attackthe_king(queens: Vec<Vec<i32>>, king: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut board = [[false; 8]; 8];
        for queen in queens {
            board[queen[0] as usize][queen[1] as usize] = true;
        }
        let directions = vec![
            vec![0, 1],
            vec![0, -1],
            vec![1, 0],
            vec![-1, 0],
            vec![1, 1],
            vec![1, -1],
            vec![-1, 1],
            vec![-1, -1],
        ];
        for direction in directions {
            let mut x = king[0];
            let mut y = king[1];
            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                if board[x as usize][y as usize] {
                    result.push(vec![x, y]);
                    break;
                }
                x += direction[0];
                y += direction[1];
            }
        }
        return result;
    }
}
