fn main() {
    assert_eq!(
        Solution::diagonal_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        25
    );
    assert_eq!(
        Solution::diagonal_sum(vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1]
        ]),
        8
    );
    assert_eq!(Solution::diagonal_sum(vec![vec![5]]), 5);
}

struct Solution;
impl Solution {
    pub fn diagonal_sum(mat: Vec<Vec<i32>>) -> i32 {
        let mut sum = 0;
        for m in 0..mat.len() {
            sum += mat[m][m];
            sum += mat[m][mat.len() - 1 - m];
        }
        return sum
            - if mat.len() % 2 == 1 {
                mat[mat.len() / 2][mat.len() / 2]
            } else {
                0
            };
    }
}
