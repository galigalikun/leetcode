fn main() {
    assert_eq!(
        Solution::reconstruct_matrix(2, 1, vec![1, 1, 1]),
        vec![[1, 1, 0], [0, 0, 1]]
    );
    assert_eq!(
        Solution::reconstruct_matrix(2, 3, vec![2, 2, 1, 1]),
        vec![vec![]]
    );
}

struct Solution;
impl Solution {
    pub fn reconstruct_matrix(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut upper = upper;
        let mut lower = lower;
        let mut colsum = colsum;
        let mut result = vec![vec![0; colsum.len()]; 2];
        for i in 0..colsum.len() {
            if colsum[i] == 2 {
                result[0][i] = 1;
                result[1][i] = 1;
                upper -= 1;
                lower -= 1;
                colsum[i] = 0;
            }
        }
        for i in 0..colsum.len() {
            if colsum[i] == 1 {
                if upper > 0 {
                    result[0][i] = 1;
                    upper -= 1;
                } else if lower > 0 {
                    result[1][i] = 1;
                    lower -= 1;
                } else {
                    return vec![];
                }
            }
        }
        if upper == 0 && lower == 0 {
            return result;
        }
        return vec![];
    }
}
