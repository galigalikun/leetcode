fn main() {
    assert_eq!(
        Solution::check_straight_line(vec![
            vec![1, 2],
            vec![2, 3],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![6, 7]
        ]),
        true
    );
    assert_eq!(
        Solution::check_straight_line(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 4],
            vec![4, 5],
            vec![5, 6],
            vec![7, 7]
        ]),
        false
    );
    assert_eq!(
        Solution::check_straight_line(vec![vec![1, 2], vec![2, 3]]),
        true
    );
}

struct Solution;
impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let x = coordinates[0][0];
        let y = coordinates[0][1];
        let mut x1 = coordinates[1][0];
        let mut y1 = coordinates[1][1];
        let mut x2 = if coordinates.len() > 2 {
            coordinates[2][0]
        } else {
            coordinates[1][0]
        };
        let mut y2 = if coordinates.len() > 2 {
            coordinates[2][1]
        } else {
            coordinates[1][1]
        };
        let a = (y1 - y) as f64 / (x1 - x) as f64;
        let mut b = (y2 - y1) as f64 / (x2 - x1) as f64;
        if a == b {
            for i in 3..coordinates.len() {
                x2 = coordinates[i][0];
                y2 = coordinates[i][1];
                b = (y2 - y1) as f64 / (x2 - x1) as f64;
                if a != b {
                    return false;
                }
                x1 = x2;
                y1 = y2;
            }
            return true;
        }
        return false;
    }
}
