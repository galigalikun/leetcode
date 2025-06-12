fn main() {
    assert_eq!(
        Solution::get_biggest_three(vec![
            vec![3, 4, 5, 1, 3],
            vec![3, 3, 4, 2, 3],
            vec![20, 30, 200, 40, 10],
            vec![1, 5, 5, 4, 1],
            vec![4, 3, 2, 2, 5]
        ]),
        vec![228, 216, 211]
    );
    assert_eq!(
        Solution::get_biggest_three(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![20, 9, 8]
    );
    assert_eq!(Solution::get_biggest_three(vec![vec![7, 7, 7]]), vec![7]);
}

struct Solution;
impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sums = vec![];
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                // size 0 rhombus (single cell)
                sums.push(grid[i][j]);
                // try larger rhombuses
                let mut k = 1;
                loop {
                    if i >= k && i + k < m && j >= k && j + k < n {
                        let mut sum = 0;
                        // four sides
                        for d in 0..k {
                            sum += grid[i - d][j + d]; // top-right
                            sum += grid[i + d][j + d]; // bottom-right
                            sum += grid[i + d][j - d]; // bottom-left
                            sum += grid[i - d][j - d]; // top-left
                        }
                        // four corners are counted twice, subtract them
                        sum -= grid[i - k][j];
                        sum -= grid[i][j + k];
                        sum -= grid[i + k][j];
                        sum -= grid[i][j - k];
                        sums.push(sum);
                        k += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        use std::collections::HashSet;
        let mut set: Vec<i32> = HashSet::<i32>::from_iter(sums.into_iter())
            .into_iter()
            .collect();
        set.sort_unstable_by(|a, b| b.cmp(a));
        set.truncate(3);
        set
    }
}
