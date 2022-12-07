fn main() {
    assert_eq!(Solution::surface_area(vec![vec![1,2],vec![3,4]]), 34);
    assert_eq!(Solution::surface_area(vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]]), 32);
    assert_eq!(Solution::surface_area(vec![vec![2,2,2],vec![2,1,2],vec![2,2,2]]), 46);
}

struct Solution;
impl Solution {
    pub fn surface_area(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                ans += 6*grid[i][j] - 1;
                if j > 0 {
                    ans -= grid[i][j-1];
                }
                if i > 0 {
                    ans -= grid[i-1][j];
                }
            }

        }
        return ans;
    }
}
