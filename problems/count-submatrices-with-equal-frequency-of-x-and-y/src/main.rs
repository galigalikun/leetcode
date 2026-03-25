fn main() {
    assert_eq!(Solution::number_of_submatrices(vec![vec!['X','Y','.'],vec!['Y','.','.']]), 3);
    assert_eq!(Solution::number_of_submatrices(vec![vec!['X','X'],vec!['X','Y']]), 0);
    assert_eq!(Solution::number_of_submatrices(vec![vec!['.','.'],vec!['.','.']]), 0);
}

struct Solution;
impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 'X' {
                    return 1;
                }
            }
        }
        return 0;
    }
}
