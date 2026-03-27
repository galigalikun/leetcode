fn main() {
    assert_eq!(
        Solution::construct_product_matrix(
            vec![[1, 2], [3, 4]].iter().map(|x| x.to_vec()).collect()
        ),
        vec![[24, 12], [8, 6]]
    );
    assert_eq!(
        Solution::construct_product_matrix(
            vec![[12345], [2], [1]].iter().map(|x| x.to_vec()).collect()
        ),
        vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]
    );
}

struct Solution;
impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                ans[i][j] = grid[i][j] * grid[j][i];
            }
        }
        ans
    }
}
