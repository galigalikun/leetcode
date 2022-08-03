fn main() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            [1, 0, 0, 1],
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
        return -1;
    }
}
