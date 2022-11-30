fn main() {
    assert_eq!(
        Solution::spiral_matrix_iii(1, 4, 0, 0),
        vec![[0, 0], [0, 1], [0, 2], [0, 3]]
    );
    assert_eq!(
        Solution::spiral_matrix_iii(5, 6, 1, 4),
        vec![
            [1, 4],
            [1, 5],
            [2, 5],
            [2, 4],
            [2, 3],
            [1, 3],
            [0, 3],
            [0, 4],
            [0, 5],
            [3, 5],
            [3, 4],
            [3, 3],
            [3, 2],
            [2, 2],
            [1, 2],
            [0, 2],
            [4, 5],
            [4, 4],
            [4, 3],
            [4, 2],
            [4, 1],
            [3, 1],
            [2, 1],
            [1, 1],
            [0, 1],
            [4, 0],
            [3, 0],
            [2, 0],
            [1, 0],
            [0, 0]
        ]
    );
}

struct Solution;
impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        return vec![];
    }
}
