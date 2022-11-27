fn main() {
    assert_eq!(Solution::projection_area(vec![vec![1, 2], vec![3, 4]]), 17);
    assert_eq!(Solution::projection_area(vec![vec![2]]), 5);
    assert_eq!(Solution::projection_area(vec![vec![1, 0], vec![0, 2]]), 8);
}

struct Solution;
impl Solution {
    pub fn projection_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut a = vec![0; grid.len()];
        let mut b = vec![0; grid[0].len()];
        let mut ans = 0;
        for x in 0..grid.len() {
            for y in 0..grid[0].len() {
                a[x] = std::cmp::max(a[x], grid[x][y]);
                b[y] = std::cmp::max(b[y], grid[x][y]);
                if grid[x][y] > 0 {
                    ans += 1;
                }
            }
        }

        ans += a.iter().fold(0, |s, p| s + p) + b.iter().fold(0, |s, p| s + p);
        return ans;
    }
}
