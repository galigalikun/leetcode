fn main() {
    assert_eq!(
        Solution::minimum_operations(vec![vec![1, 0, 2], vec![1, 0, 2]]),
        0
    );
    assert_eq!(
        Solution::minimum_operations(vec![vec![1, 1, 1], vec![0, 0, 0]]),
        3
    );
    assert_eq!(
        Solution::minimum_operations(vec![vec![1], vec![2], vec![3]]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn minimum_operations(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        for j in 0..n {
            let mut expected = 1;
            for i in 0..m {
                if grid[i][j] == expected {
                    expected += 1;
                }
            }
            if expected - 1 == m as i32 {
                return (j as i32 + n as i32 - expected) % n as i32;
            }
        }
        return 0;
    }
}
