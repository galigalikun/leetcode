fn main() {
    assert_eq!(
        Solution::is_boomerang(vec![vec![1, 1], vec![2, 3], vec![3, 2]]),
        true
    );
    assert_eq!(
        Solution::is_boomerang(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn is_boomerang(points: Vec<Vec<i32>>) -> bool {
        let x1 = points[0][0];
        let y1 = points[0][1];
        let x2 = points[1][0];
        let y2 = points[1][1];
        return (x1 - x2) * (points[2][1] - y2) != (y1 - y2) * (points[2][0] - x2);
    }
}
