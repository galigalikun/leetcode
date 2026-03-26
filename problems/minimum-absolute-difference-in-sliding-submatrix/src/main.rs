fn main() {
    assert_eq!(
        Solution::min_abs_diff(
            vec![[1, 8], [3, -2]].iter().map(|&x| x.to_vec()).collect(),
            2
        ),
        vec![[2]]
    );
    assert_eq!(
        Solution::min_abs_diff(vec![[3, -1]].iter().map(|&x| x.to_vec()).collect(), 1),
        vec![[0, 0]]
    );
    assert_eq!(
        Solution::min_abs_diff(
            vec![[1, -2, 3], [2, 3, 5]]
                .iter()
                .map(|&x| x.to_vec())
                .collect(),
            2
        ),
        vec![[1, 2]]
    );
}

struct Solution;
impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let mut min_diff = i32::MAX;
                for x in 0..m {
                    for y in 0..n {
                        if (i as i32 - x as i32).abs() + (j as i32 - y as i32).abs() <= k {
                            min_diff = min_diff.min((grid[i][j] - grid[x][y]).abs());
                        }
                    }
                }
                ans[i][j] = min_diff;
            }
        }
        ans
    }
}
