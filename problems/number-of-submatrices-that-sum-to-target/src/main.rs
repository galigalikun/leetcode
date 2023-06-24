fn main() {
    assert_eq!(
        Solution::num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0),
        4
    );
    assert_eq!(
        Solution::num_submatrix_sum_target(vec![vec![1, -1], vec![-1, 1]], 0),
        5
    );
    assert_eq!(Solution::num_submatrix_sum_target(vec![vec![904]], 0), 0);
}

struct Solution;
impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let mut count = 0;
        let mut sum = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + matrix[i][j];
            }
        }
        for i in 0..matrix.len() {
            for j in i..matrix.len() {
                let mut map = std::collections::HashMap::new();
                map.insert(0, 1);
                for k in 0..matrix[0].len() {
                    let diff = sum[j + 1][k + 1] - sum[i][k + 1];
                    if let Some(&v) = map.get(&(diff - target)) {
                        count += v;
                    }
                    *map.entry(diff).or_insert(0) += 1;
                }
            }
        }
        return count;
    }
}
