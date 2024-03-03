fn main() {
    assert_eq!(
        Solution::lucky_numbers(vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]]),
        vec![15]
    );
    assert_eq!(
        Solution::lucky_numbers(vec![
            vec![1, 10, 4, 2],
            vec![9, 3, 8, 7],
            vec![15, 16, 17, 12]
        ]),
        vec![12]
    );
    assert_eq!(
        Solution::lucky_numbers(vec![vec![7, 8], vec![1, 2]]),
        vec![7]
    );
}

struct Solution;
impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut min_row = vec![];
        let mut max_col = vec![];
        for row in matrix.iter() {
            let min = row.iter().min().unwrap();
            min_row.push(min);
        }
        for col in 0..matrix[0].len() {
            let mut max = matrix[0][col];
            for row in 0..matrix.len() {
                if matrix[row][col] > max {
                    max = matrix[row][col];
                }
            }
            max_col.push(max);
        }
        for i in 0..min_row.len() {
            if max_col.contains(&min_row[i]) {
                return vec![min_row[i].clone()];
            }
        }
        return vec![];
    }
}
