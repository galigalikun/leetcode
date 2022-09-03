fn main() {
    assert_eq!(
        Solution::largest_triangle_area(vec![
            vec![0, 0],
            vec![0, 1],
            vec![1, 0],
            vec![0, 2],
            vec![2, 0]
        ]),
        2.0
    );
    assert_eq!(
        Solution::largest_triangle_area(vec![vec![1, 0], vec![0, 0], vec![0, 1]]),
        0.5
    );
    assert_eq!(
        Solution::largest_triangle_area(vec![vec![4, 6], vec![6, 5], vec![3, 1]]),
        5.5
    );
}

struct Solution {}
impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut x = (0, std::i32::MAX);
        let mut y = (0, std::i32::MAX);
        for point in points {
            x.0 = std::cmp::max(x.0, point[0]);
            x.1 = std::cmp::min(x.1, point[0]);
            y.0 = std::cmp::max(y.0, point[1]);
            y.1 = std::cmp::min(y.1, point[1]);
        }
        return ((x.0 - x.1) * (y.0 - y.1)) as f64 * 0.5;
    }
}
