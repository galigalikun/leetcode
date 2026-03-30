fn main() {
    assert_eq!(
        Solution::are_similar(
            vec![[1, 2, 3], [4, 5, 6], [7, 8, 9]]
                .iter()
                .map(|x| x.to_vec())
                .collect(),
            4
        ),
        false
    );
    assert_eq!(
        Solution::are_similar(
            vec![[1, 2, 1, 2], [5, 5, 5, 5], [6, 3, 6, 3]]
                .iter()
                .map(|x| x.to_vec())
                .collect(),
            2
        ),
        true
    );
    assert_eq!(
        Solution::are_similar(vec![[2, 2], [2, 2]].iter().map(|x| x.to_vec()).collect(), 3),
        true
    );
}

struct Solution;
impl Solution {
    pub fn are_similar(mat: Vec<Vec<i32>>, k: i32) -> bool {
        let mut count = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count <= k
    }
}
