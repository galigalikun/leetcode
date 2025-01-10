fn main() {
    assert_eq!(
        Solution::find_ball(vec![
            vec![1, 1, 1, -1, -1],
            vec![1, 1, 1, -1, -1],
            vec![-1, -1, -1, 1, 1],
            vec![1, 1, 1, 1, -1],
            vec![-1, -1, -1, -1, -1]
        ]),
        vec![1, -1, -1, -1, -1]
    );
    assert_eq!(Solution::find_ball(vec![vec![-1]]), vec![-1])
}

struct Solution;
impl Solution {
    pub fn find_ball(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![-1; n];
        for i in 0..n {
            let mut x = i;
            for j in 0..m {
                if grid[j][x] == 1 {
                    if x + 1 < n && grid[j][x + 1] == 1 {
                        x += 1;
                    } else {
                        break;
                    }
                } else {
                    if x > 0 && grid[j][x - 1] == -1 {
                        x -= 1;
                    } else {
                        break;
                    }
                }
                if j == m - 1 {
                    ans[i] = x as i32;
                }
            }
        }
        ans
    }
}
