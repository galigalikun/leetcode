fn main() {
    assert_eq!(Solution::surface_area(vec![vec![1, 2], vec![3, 4]]), 34);
    assert_eq!(
        Solution::surface_area(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        32
    );
    assert_eq!(
        Solution::surface_area(vec![vec![2, 2, 2], vec![2, 1, 2], vec![2, 2, 2]]),
        46
    );
}

struct Solution;
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ans = 0;

        for i in 0..n {
            for j in 0..n {
                let h = grid[i][j];
                if h == 0 {
                    continue;
                }

                ans += 2;

                let north = if i > 0 { grid[i - 1][j] } else { 0 };
                let south = if i + 1 < n { grid[i + 1][j] } else { 0 };
                let west = if j > 0 { grid[i][j - 1] } else { 0 };
                let east = if j + 1 < n { grid[i][j + 1] } else { 0 };

                ans += (h - north).max(0);
                ans += (h - south).max(0);
                ans += (h - west).max(0);
                ans += (h - east).max(0);
            }
        }

        ans
    }
}
