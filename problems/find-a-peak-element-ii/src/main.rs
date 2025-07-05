fn main() {
    assert_eq!(
        Solution::find_peak_grid(vec![vec![1, 4], vec![3, 2]]),
        vec![0, 1]
    );
    assert_eq!(
        Solution::find_peak_grid(vec![vec![10,20,15],vec![21,30,14],vec![7,16,32]]),
        vec![1, 1]
    );
}

struct Solution;
impl Solution {
    fn find_peak_grid_helper(mat: &Vec<Vec<i32>>, left: i32, right: i32) -> Vec<i32> {
        if left > right {
            return vec![];
        }

        let mid = (left + right) / 2;
        let mut max_row = 0;
        let mut max_value = mat[0][mid as usize];

        for i in 0..mat.len() {
            if mat[i][mid as usize] > max_value {
                max_value = mat[i][mid as usize];
                max_row = i;
            }
        }

        let left_value = if mid > 0 { mat[max_row][(mid - 1) as usize] } else { i32::MIN };
        let right_value = if mid < mat[0].len() as i32 - 1 { mat[max_row][(mid + 1) as usize] } else { i32::MIN };

        if max_value > left_value && max_value > right_value {
            return vec![max_row as i32, mid];
        } else if left_value > max_value {
            return Solution::find_peak_grid_helper(mat, left, mid - 1);
        } else {
            return Solution::find_peak_grid_helper(mat, mid + 1, right);
        }
    }
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        return Solution::find_peak_grid_helper(&mat, 0, mat.len() as i32 - 1);
    }
}
