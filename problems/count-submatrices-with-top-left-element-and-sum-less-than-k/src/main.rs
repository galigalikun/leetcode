fn main() {
    assert_eq!(
        Solution::count_submatrices(
            vec![[7, 6, 3], [6, 6, 1]]
                .iter()
                .map(|&row| row.to_vec())
                .collect(),
            18
        ),
        4
    );
    assert_eq!(
        Solution::count_submatrices(
            vec![[7, 2, 9], [1, 5, 0], [2, 6, 6]]
                .iter()
                .map(|&row| row.to_vec())
                .collect(),
            20
        ),
        6
    );
}

struct Solution;
impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = 0;
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            for j in 0..n {
                let mut sum = 0;
                for x in i..m {
                    for y in j..n {
                        sum += grid[x][y];
                        if sum == k {
                            ans += 1;
                        }
                    }
                }
            }
        }
        ans
    }
}
