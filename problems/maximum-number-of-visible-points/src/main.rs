fn main() {
    assert_eq!(
        Solution::visible_points(vec![vec![2, 1], vec![2, 2], vec![3, 3]], 90, vec![1, 1]),
        3
    );
    assert_eq!(
        Solution::visible_points(
            vec![vec![2, 1], vec![2, 2], vec![3, 4], vec![1, 1]],
            90,
            vec![1, 1]
        ),
        4
    );
    assert_eq!(
        Solution::visible_points(vec![vec![1, 0], vec![2, 1]], 13, vec![1, 1]),
        1
    );
}

struct Solution;
impl Solution {
    pub fn visible_points(points: Vec<Vec<i32>>, angle: i32, location: Vec<i32>) -> i32 {
        use std::f64::consts::PI;
        let mut angles = vec![];
        let mut same = 0;
        for point in &points {
            let x = point[0] - location[0];
            let y = point[1] - location[1];
            if x == 0 && y == 0 {
                same += 1;
                continue;
            }
            let mut angle = (y as f64).atan2(x as f64) * 180.0 / PI;
            if angle < 0.0 {
                angle += 360.0;
            }
            angles.push(angle);
        }
        angles.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut angles2 = angles.clone();
        for angle in &angles {
            angles2.push(angle + 360.0);
        }
        let mut max = 0;
        let mut j = 0;
        for i in 0..angles.len() {
            while j < angles2.len() && angles2[j] - angles[i] <= angle as f64 {
                j += 1;
            }
            max = max.max(j - i);
        }
        max as i32 + same
    }
}
