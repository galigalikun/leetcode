fn main() {
    assert_eq!(
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 3, 6, 9, 8, 7, 4, 5]
    );
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12]
        ]),
        vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
    );
    assert_eq!(Solution::spiral_order(vec![]), vec![]);
    assert_eq!(Solution::spiral_order(vec![vec![1]]), vec![1]);
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![5],
            vec![6],
            vec![7],
            vec![8],
            vec![9],
            vec![10]
        ]),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    );
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16]
        ]),
        vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10]
    );
    assert_eq!(
        Solution::spiral_order(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25]
        ]),
        vec![
            1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12,
            13
        ]
    );
}

pub struct Solution {}
impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 {
            return vec![];
        }
        let mut result = Vec::new();
        let mut i = 0;
        let mut l = 0;
        let mut r = 0;
        let mut t = 0;
        let mut b = 0;
        let area = matrix.len() * matrix[0].len();
        loop {
            if i == 0 {
                for row in t..matrix.len() - b {
                    if row == t {
                        for col in l..matrix[row].len() - r {
                            result.push(matrix[row][col]);
                        }
                    } else if row == matrix.len() - 1 - b {
                        for col in (l..matrix[row].len() - r).rev() {
                            result.push(matrix[row][col]);
                        }
                    } else {
                        result.push(matrix[row][matrix[row].len() - 1 - r]);
                    }
                }
                t += 1;
                b += 1;
                r += 1;
            } else if i % 2 == 0 {
                for row in t..matrix.len() - b {
                    if row == matrix.len() - 1 - b {
                        for col in (l..matrix[row].len() - r).rev() {
                            result.push(matrix[row][col]);
                        }
                    } else {
                        result.push(matrix[row][matrix[row].len() - 1 - r]);
                    }
                }
                r += 1;
                b += 1;
            } else {
                for row in (t..matrix.len() - b).rev() {
                    if row == t {
                        for col in l..matrix[row].len() - r {
                            result.push(matrix[row][col]);
                        }
                    } else {
                        result.push(matrix[row][l]);
                    }
                }
                l += 1;
                t += 1;
            }

            i += 1;

            if result.len() >= area {
                break;
            }
        }

        return result;
    }
}
