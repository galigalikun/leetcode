fn main() {
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 0],
                vec![1, 0, 0, 0, 0],
                vec![1, 1, 0, 0, 0],
                vec![1, 1, 1, 1, 1]
            ],
            3
        ),
        vec![2, 0, 3]
    );
    assert_eq!(
        Solution::k_weakest_rows(
            vec![
                vec![1, 0, 0, 0],
                vec![1, 1, 1, 1],
                vec![1, 0, 0, 0],
                vec![1, 0, 0, 0]
            ],
            2
        ),
        vec![0, 2]
    );
}

struct Solution;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..mat.len() {
            let mut sum = 0;
            for j in 0..mat[i].len() {
                sum += mat[i][j];
            }
            result.push((sum, i));
        }
        result.sort();
        let mut ans = vec![];
        for i in 0..k {
            ans.push(result[i as usize].1 as i32);
        }
        return ans;
    }
}
