fn main() {
    assert_eq!(Solution::odd_cells(2, 3, vec![vec![0, 1], vec![1, 1]]), 6);
    assert_eq!(Solution::odd_cells(2, 2, vec![vec![1, 1], vec![0, 0]]), 0);
}

struct Solution;
impl Solution {
    pub fn odd_cells(m: i32, n: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![0; n as usize]; m as usize];
        for i in indices {
            for j in 0..n {
                matrix[i[0] as usize][j as usize] += 1;
            }
            for j in 0..m {
                matrix[j as usize][i[1] as usize] += 1;
            }
        }
        let mut count = 0;
        for i in matrix {
            for j in i {
                if j % 2 == 1 {
                    count += 1;
                }
            }
        }
        return count;
    }
}
