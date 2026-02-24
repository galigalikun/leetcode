fn main() {
    assert_eq!(
        Solution::separate_squares(
            vec![[0, 0, 1], [2, 2, 1]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        1.0
    );
    assert_eq!(
        Solution::separate_squares(
            vec![[0, 0, 2], [1, 1, 1]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        1.16667
    );
}

struct Solution;
impl Solution {
    pub fn separate_squares(squares: Vec<Vec<i32>>) -> f64 {
        let mut total_area = 0.0;
        for square in squares {
            let side_length = square[2] as f64;
            total_area += side_length * side_length;
        }
        return total_area;
    }
}
