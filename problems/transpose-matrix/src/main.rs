fn main() {
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]]
    );
    assert_eq!(
        Solution::transpose(vec![vec![1, 2, 3], vec![4, 5, 6]]),
        vec![[1, 4], [2, 5], [3, 6]]
    );
}

struct Solution;
impl Solution {
    pub fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        for i in 0..matrix[0].len() {
            let mut arr = vec![];
            for j in 0..matrix.len() {
                arr.push(matrix[j][i]);
            }
            ans.push(arr);
        }
        return ans;
    }
}
