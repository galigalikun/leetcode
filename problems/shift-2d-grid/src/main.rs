fn main() {
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![[9, 1, 2], [3, 4, 5], [6, 7, 8]]
    );
    assert_eq!(
        Solution::shift_grid(
            vec![
                vec![3, 8, 1, 9],
                vec![19, 7, 2, 5],
                vec![4, 6, 11, 10],
                vec![12, 0, 21, 13]
            ],
            4
        ),
        vec![[12, 0, 21, 13], [3, 8, 1, 9], [19, 7, 2, 5], [4, 6, 11, 10]]
    );
    assert_eq!(
        Solution::shift_grid(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 9),
        vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
    );
    assert_eq!(Solution::shift_grid(vec![vec![100]], 0), vec![[100]]);
}

struct Solution;
impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        if k == 0 {
            return grid;
        }
        let mut grid = grid;
        let mut k = k;
        let mut result = vec![vec![0; grid[0].len()]; grid.len()];
        while k > 0 {
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if j == grid[0].len() - 1 {
                        if i == grid.len() - 1 {
                            result[0][0] = grid[i][j];
                        } else {
                            result[i + 1][0] = grid[i][j];
                        }
                    } else {
                        result[i][j + 1] = grid[i][j];
                    }
                }
            }
            grid = result.clone();
            k -= 1;
        }
        return result;
    }
}
