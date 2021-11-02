fn main() {
    assert_eq!(
        Solution::generate_matrix(3),
        vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]]
    );
    assert_eq!(Solution::generate_matrix(2), vec![[1, 2], [4, 3]]);
    assert_eq!(
        Solution::generate_matrix(4),
        vec![
            [1, 2, 3, 4],
            [12, 13, 14, 5],
            [11, 16, 15, 6],
            [10, 9, 8, 7]
        ]
    );
    assert_eq!(
        Solution::generate_matrix(5),
        vec![
            [1, 2, 3, 4, 5],
            [16, 17, 18, 19, 6],
            [15, 24, 25, 20, 7],
            [14, 23, 22, 21, 8],
            [13, 12, 11, 10, 9]
        ]
    );
}

pub struct Solution {}
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; n as usize]; n as usize];

        let mut no = 1;
        let mut t = 0;
        let mut b = 0;
        let mut l = 0;
        let mut r = 0;
        for i in 0..n {
            if i == 0 {
                for j in 0..n {
                    result[i as usize][j as usize] = no;
                    no += 1;
                }
                for j in 1..n {
                    result[j as usize][(n - 1) as usize] = no;
                    no += 1;
                }
                for j in (0..n - 1).rev() {
                    result[(n - 1) as usize][j as usize] = no;
                    no += 1;
                }
                t += 1;
                b += 1;
                r += 1;
            } else if i % 2 == 0 {
                for row in t..n - b {
                    if row == n - 1 - b {
                        for j in (l..n - r).rev() {
                            result[row as usize][j as usize] = no;
                            no += 1;
                        }
                    } else {
                        result[row as usize][(n - 1 - r) as usize] = no;
                        no += 1;
                    }
                }
                r += 1;
                b += 1;
            } else {
                for row in (t..n - b).rev() {
                    if row == t {
                        for j in l..n - r {
                            result[row as usize][j as usize] = no;
                            no += 1;
                        }
                    } else {
                        result[row as usize][l as usize] = no;
                        no += 1;
                    }
                }
                l += 1;
                t += 1;
            }
            if no >= n * n {
                break;
            }
        }

        return result;
    }
}
