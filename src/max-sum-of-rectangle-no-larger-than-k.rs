fn main() {
    assert_eq!(
        Solution::max_sum_submatrix(vec![vec![1, 0, 1], vec![0, -2, 3]], 2),
        2
    );

    assert_eq!(Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 3), 3);

    assert_eq!(Solution::max_sum_submatrix(vec![vec![2,2,-1]], 0), -1);
}

pub struct Solution {}
// https://www.geeksforgeeks.org/maximum-sum-not-exceeding-k-possible-for-any-rectangle-of-a-matrix/
use std::collections::HashSet;
impl Solution {
    fn helper(sum: Vec<i32>, k: i32, row: usize) -> i32 {
        let mut cur_sum = 0;
        let mut cur_max = std::i32::MIN;
        let mut sum_set = HashSet::new();
        sum_set.insert(0);
        for r in 0..row {
            cur_sum += sum[r];

            if let Some(n) = sum_set.get(&(cur_sum - k)) {
                cur_max = std::cmp::max(cur_max, cur_sum - n);
            }

            sum_set.insert(cur_sum);
        }
        return cur_max;
    }
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();
        let mut result = std::i32::MIN;

        for i in 0..col {
            let mut sum = vec![0; row];
            for j in i..col {
                for r in 0..row {
                    sum[r] += matrix[r][j];
                }
                let cur_max = Solution::helper(sum.clone(), k, row);

                result = std::cmp::max(result, cur_max);
            }
        }
        return result;
    }
}
