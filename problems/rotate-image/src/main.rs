fn main() {
    let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);

    matrix = vec![
        vec![5, 1, 9, 11],
        vec![2, 4, 8, 10],
        vec![13, 3, 6, 7],
        vec![15, 14, 12, 16],
    ];
    Solution::rotate(&mut matrix);
    assert_eq!(
        matrix,
        vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11]
        ]
    );

    matrix = vec![vec![1]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![1]]);

    matrix = vec![vec![1, 2], vec![3, 4]];
    Solution::rotate(&mut matrix);
    assert_eq!(matrix, vec![vec![3, 1], vec![4, 2]]);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut tmp: HashMap<(usize, usize), i32> = HashMap::new();
        for x in 0..matrix.len() {
            for y in 0..matrix[x].len() {
                /*
                [0][0] => [2][0]
                [2][0] => [2][2]
                [2][2] => [0][2]
                [0][2] => [0][0]

                [0][1] => [1][0]
                [1][0] => [2][1]
                [2][1] => [1][2]
                [1][2] => [0][1]


                */
                /*
                    [0][0] => [2][0]
                    [0][1] => [1][0]
                    [0][2] => [0][0]
                    [1][0] => [2][1]
                    [1][1] => [1][1]
                    [1][2] => [0][1]
                    [2][0] => [2][2]
                    [2][1] => [1][2]
                    [2][2] => [0][2]
                */
                tmp.insert((x, y), matrix[x][y]);
                let xx = matrix[x].len() - 1 - y;
                let yy = x;
                if let Some(&v) = tmp.get(&(xx, yy)) {
                    matrix[x][y] = v;
                } else {
                    matrix[x][y] = matrix[xx][yy];
                }
            }
        }
    }
}
