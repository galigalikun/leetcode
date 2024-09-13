fn main() {
    assert_eq!(
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
        1
    );
    assert_eq!(
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![0; mat.len()];
        let mut col = vec![0; mat[0].len()];
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                row[i] += mat[i][j];
                col[j] += mat[i][j];
            }
        }
        let mut count = 0;
        for i in row.iter() {
            for j in col.iter() {
                if *i == 1 && *j == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}
