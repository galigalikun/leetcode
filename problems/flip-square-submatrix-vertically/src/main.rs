fn main() {
    assert_eq!(
        Solution::reverse_submatrix(
            vec![
                [1, 2, 3, 4],
                [5, 6, 7, 8],
                [9, 10, 11, 12],
                [13, 14, 15, 16]
            ]
            .iter()
            .map(|row| row.iter().map(|&x| x % 2).collect())
            .collect(),
            1,
            0,
            3
        ),
        vec![
            [1, 2, 3, 4],
            [13, 14, 15, 8],
            [9, 10, 11, 12],
            [5, 6, 7, 16]
        ]
    );
    assert_eq!(
        Solution::reverse_submatrix(
            vec![[3, 4, 2, 3], [2, 3, 4, 2]]
                .iter()
                .map(|row| row.iter().map(|&x| x % 2).collect())
                .collect(),
            0,
            2,
            2
        ),
        vec![[3, 4, 4, 2], [2, 3, 2, 3]]
    );
}

struct Solution;
impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut grid = grid;
        let n = grid.len() as i32;
        let m = grid[0].len() as i32;
        for i in 0..k {
            for j in 0..k {
                if x + i < n && y + j < m {
                    grid[(x + i) as usize][(y + j) as usize] =
                        1 - grid[(x + i) as usize][(y + j) as usize];
                }
            }
        }
        grid
    }
}
