fn main() {
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 3, 4]
    );
}

struct Solution {}
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let m = mat.len();
        let n = mat[0].len();
        let mut result = vec![0; m * n];
        let mut i = 0;
        let mut j = 0;
        let mut turn = 0;
        for idx in 0..result.len() {
            result[idx] = mat[i][j];
            if turn == 0 {
                j += 1;
                if j >= n {
                    j -= 1;
                    i += 1;
                    turn = 1;
                } else if i == 0 {
                    turn = 1;
                } else {
                    i -= 1;
                }
            } else {
                i += 1;
                if i >= m {
                    j += 1;
                    i -= 1;
                    turn = 0;
                } else if j == 0 {
                    turn = 0;
                } else {
                    j -= 1;
                }
            }
        }
        return result;
    }
}
