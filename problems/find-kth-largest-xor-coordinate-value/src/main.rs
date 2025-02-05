fn main() {
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 1),
        7
    );
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 2),
        5
    );
    assert_eq!(
        Solution::kth_largest_value(vec![vec![5, 2], vec![1, 6]], 3),
        4
    );
}

struct Solution;
impl Solution {
    pub fn kth_largest_value(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut xor = vec![vec![0; m + 1]; n + 1];
        let mut res = vec![];
        for i in 1..=n {
            for j in 1..=m {
                xor[i][j] =
                    xor[i - 1][j] ^ xor[i][j - 1] ^ xor[i - 1][j - 1] ^ matrix[i - 1][j - 1];
                res.push(xor[i][j]);
            }
        }
        res.sort_unstable();
        res.reverse();
        if k as usize <= res.len() {
            return res[k as usize - 1];
        }
        return 0;
    }
}
