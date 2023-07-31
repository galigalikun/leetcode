fn main() {
    assert_eq!(
        Solution::max_abs_val_expr(vec![1, 2, 3, 4], vec![-1, 4, 5, 6]),
        13
    );
    assert_eq!(
        Solution::max_abs_val_expr(vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4]),
        20
    );
}

struct Solution;
impl Solution {
    pub fn max_abs_val_expr(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..arr1.len() {
            for j in 0..arr1.len() {
                let val = (arr1[i] - arr1[j]).abs()
                    + (arr2[i] - arr2[j]).abs()
                    + (i as i32 - j as i32).abs();
                if val > max {
                    max = val;
                }
            }
        }
        return max;
    }
}
