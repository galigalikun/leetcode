fn main() {
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 1, 4),
        vec![[1, 2, 3, 4]]
    );
    assert_eq!(
        Solution::matrix_reshape(vec![vec![1, 2], vec![3, 4]], 2, 4),
        vec![[1, 2], [3, 4]]
    );
}

struct Solution {}
impl Solution {
    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        if mat.len() * mat[0].len() != (r * c) as usize {
            return mat;
        }
        let mut ans = vec![vec![0; mat.len() * mat[0].len() / r as usize]; r as usize];
        let mut row = 0;
        let mut cell = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                ans[row][cell] = mat[i][j];
                cell += 1;
                if cell == c as usize {
                    row += 1;
                    cell = 0;
                }
            }
        }

        return ans;
    }
}
