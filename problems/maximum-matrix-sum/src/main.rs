fn main() {
    assert_eq!(Solution::max_matrix_sum(vec![vec![1, -1], vec![-1, 1]]), 4);
    assert_eq!(
        Solution::max_matrix_sum(vec![vec![1, 2, 3], vec![-1, -2, -3], vec![1, 2, 3]]),
        16
    );
}

struct Solution;
impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let mut neg = 0;
        let mut min = i32::MAX;
        let mut sum = 0i64;
        for row in matrix {
            for v in row {
                if v < 0 {
                    neg += 1;
                }
                min = min.min(v.abs());
                sum += v.abs() as i64;
            }
        }
        if neg % 2 == 0 {
            return sum;
        }
        sum -= min as i64 * 2;
        return sum;
    }
}
