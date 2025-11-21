fn main() {
    assert_eq!(
        Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'B']
        ]),
        true
    );
    assert_eq!(
        Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['W', 'B', 'W'],
            vec!['B', 'W', 'B']
        ]),
        false
    );
    assert_eq!(
        Solution::can_make_square(vec![
            vec!['B', 'W', 'B'],
            vec!['B', 'W', 'W'],
            vec!['B', 'W', 'W']
        ]),
        true
    );
}

struct Solution;
impl Solution {
    pub fn can_make_square(grid: Vec<Vec<char>>) -> bool {
        let n = grid.len();
        let m = grid[0].len();
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '1' {
                    if i + 1 < n
                        && j + 1 < m
                        && grid[i + 1][j] == '1'
                        && grid[i][j + 1] == '1'
                        && grid[i + 1][j + 1] == '1'
                    {
                        continue;
                    } else {
                        return false;
                    }
                }
            }
        }
        true
    }
}
