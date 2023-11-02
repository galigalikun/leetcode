fn main() {
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![1, 1], vec![3, 4], vec![-1, 0]]),
        7
    );
    assert_eq!(
        Solution::min_time_to_visit_all_points(vec![vec![3, 2], vec![-2, 2]]),
        5
    );
}

struct Solution;
impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..points.len() - 1 {
            let x = (points[i][0] - points[i + 1][0]).abs();
            let y = (points[i][1] - points[i + 1][1]).abs();
            result += x.max(y);
        }
        return result;
    }
}
