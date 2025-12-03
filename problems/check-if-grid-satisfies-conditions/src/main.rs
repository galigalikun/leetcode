fn main() {
    assert_eq!(
        Solution::satisfies_conditions(vec![vec![1, 0, 2], vec![1, 0, 2]]),
        true
    );
    assert_eq!(
        Solution::satisfies_conditions(vec![vec![1, 1, 1], vec![0, 0, 0]]),
        false
    );
    assert_eq!(
        Solution::satisfies_conditions(vec![vec![1], vec![2], vec![3]]),
        false
    );
    assert_eq!(
        Solution::satisfies_conditions(vec![vec![1, 1, 6, 1, 4, 6, 3, 1, 0, 7]]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn satisfies_conditions(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                if i + 1 >= m {
                    break;
                }
                if grid[i][j] != grid[i + 1][j] {
                    return false;
                }
                if j + 1 >= n {
                    break;
                }
                if grid[i][j] == grid[i][j + 1] {
                    return false;
                }
            }
        }
        return true;
    }
}
