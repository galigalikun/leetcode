fn main() {
    assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![0, 1]]), 0);
    assert_eq!(Solution::count_servers(vec![vec![1, 0], vec![1, 1]]), 3);
    assert_eq!(
        Solution::count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row = vec![0; grid.len()];
        let mut col = vec![0; grid[0].len()];
        let mut total = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    row[i] += 1;
                    col[j] += 1;
                    total += 1;
                }
            }
        }
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && row[i] == 1 && col[j] == 1 {
                    total -= 1;
                }
            }
        }
        return total;
    }
}
