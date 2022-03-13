fn main() {
    let m1 = &mut vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    Solution::set_zeroes(m1);
    assert_eq!(m1, &mut vec![[1, 0, 1], [0, 0, 0], [1, 0, 1]]);
    let m2 = &mut vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
    Solution::set_zeroes(m2);
    assert_eq!(
        m2,
        &mut vec![vec![0, 0, 0, 0], vec![0, 4, 5, 0], vec![0, 3, 1, 0]]
    );
}

struct Solution {}
impl Solution {
    fn update_zeroes(matrix: &mut Vec<Vec<i32>>, x: usize, y: usize, m: usize, n: usize) {
        for a in 0..n {
            matrix[x][a] = 0;
        }
        for b in 0..m {
            matrix[b][y] = 0;
        }
    }
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let data = matrix.clone();
        let m = matrix.len();
        let n = matrix[0].len();
        for x in 0..m {
            for y in 0..n {
                if data[x][y] == 0 {
                    Solution::update_zeroes(matrix, x, y, m, n);
                }
            }
        }
    }
}
