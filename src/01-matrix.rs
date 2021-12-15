fn main() {
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]),
        vec![[0, 0, 0], [0, 1, 0], [0, 0, 0]]
    );
    assert_eq!(
        Solution::update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]),
        vec![[0, 0, 0], [0, 1, 0], [1, 2, 1]]
    );
}

struct Solution {}
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        for i in 0..mat.len() {
            ans.push(vec![]);
            for j in 0..mat[i].len() {
                if mat[i][j] == 0 {
                    ans[i].push(0);
                } else {
                    ans[i].push(1);
                }
                println!("debug {}", mat[i][j]);
            }
        }
        return ans;
    }
}
