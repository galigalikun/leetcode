fn main() {
    assert_eq!(
        Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]
    );
    assert_eq!(
        Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 2),
        vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]]
    );
}

struct Solution;
impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut sum = 0;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                sum += mat[i][j];
            }
        }
        for i in 0..mat.len() {
            let mut row = vec![];
            for _j in 0..mat[i].len() {
                row.push(sum);
            }
            result.push(row);
        }
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                let mut sum = 0;
                for x in i - k as usize..=i + k as usize {
                    if x >= mat.len() {
                        break;
                    }
                    for y in j - k as usize..=j + k as usize {
                        if y >= mat[x].len() {
                            break;
                        }
                        sum += mat[x][y];
                    }
                }
                result[i][j] = sum;
            }
        }
        return result;
    }
}
