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
// https://gist.github.com/yitonghe00/50a4f7dc459fdc506a9a62bbd5461972
impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut q = vec![];
        let mut ans = mat.clone();
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] == 0 {
                    q.push((i as i32, j as i32));
                } else {
                    ans[i][j] = std::i32::MAX;
                }
            }
        }
        while !q.is_empty() {
            let current = q.remove(0);
            for d in vec![(0, 1), (1, 0), (-1, 0), (0, -1)] {
                let i = current.0 + d.0;
                let j = current.1 + d.1;
                if i < 0
                    || j < 0
                    || i >= mat.len() as i32
                    || j >= mat[0].len() as i32
                    || ans[i as usize][j as usize]
                        <= ans[current.0 as usize][current.1 as usize] + 1
                {
                    continue;
                }
                ans[i as usize][j as usize] = ans[current.0 as usize][current.1 as usize] + 1;
                q.push((i, j));
            }
        }
        return ans;
    }
}
