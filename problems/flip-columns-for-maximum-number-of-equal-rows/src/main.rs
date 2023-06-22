fn main() {
    assert_eq!(
        Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]),
        1
    );
    assert_eq!(
        Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]),
        2
    );
    assert_eq!(
        Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let mut max = 0;
        for i in 0..matrix.len() {
            let mut count = 0;
            for j in 0..matrix.len() {
                if matrix[i] == matrix[j]
                    || matrix[i] == matrix[j].iter().map(|x| 1 - x).collect::<Vec<i32>>()
                {
                    count += 1;
                }
            }
            max = std::cmp::max(max, count);
        }
        return max;
    }
}
