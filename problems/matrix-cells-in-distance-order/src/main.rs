fn main() {
    assert_eq!(
        Solution::all_cells_dist_order(1, 2, 0, 0),
        vec![[0, 0], [0, 1]]
    );
    assert_eq!(
        Solution::all_cells_dist_order(2, 2, 0, 1),
        vec![[0, 1], [0, 0], [1, 1], [1, 0]]
    );
    assert_eq!(
        Solution::all_cells_dist_order(2, 3, 1, 2),
        vec![[1, 2], [0, 2], [1, 1], [0, 1], [1, 0], [0, 0]]
    );
}

struct Solution;
impl Solution {
    pub fn all_cells_dist_order(
        rows: i32,
        cols: i32,
        r_center: i32,
        c_center: i32,
    ) -> Vec<Vec<i32>> {
        let mut result = vec![];
        for i in 0..rows {
            for j in 0..cols {
                result.push(vec![i, j]);
            }
        }
        result.sort_by(|a, b| {
            let a_dist = (a[0] - r_center).abs() + (a[1] - c_center).abs();
            let b_dist = (b[0] - r_center).abs() + (b[1] - c_center).abs();
            a_dist.cmp(&b_dist)
        });
        return result;
    }
}
