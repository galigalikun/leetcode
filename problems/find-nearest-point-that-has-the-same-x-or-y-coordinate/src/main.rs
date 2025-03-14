fn main() {
    assert_eq!(
        Solution::nearest_valid_point(
            3,
            4,
            vec![vec![1, 2], vec![3, 1], vec![2, 4], vec![2, 3], vec![4, 4]]
        ),
        2
    );
    assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![3, 4]]), 0);
    assert_eq!(Solution::nearest_valid_point(3, 4, vec![vec![2, 3]]), -1);
}

struct Solution;
impl Solution {
    pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
        let mut min = std::i32::MAX;
        let mut index = -1;
        for i in 0..points.len() {
            let point = &points[i];
            if point[0] == x || point[1] == y {
                let distance = (point[0] - x).abs() + (point[1] - y).abs();
                if distance < min {
                    min = distance;
                    index = i as i32;
                }
            }
        }
        index
    }
}
