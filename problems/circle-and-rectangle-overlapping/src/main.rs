fn main() {
    assert_eq!(Solution::check_overlap(1, 0, 0, 1, -1, 3, 1), true);
    assert_eq!(Solution::check_overlap(1, 1, 1, 1, -3, 2, -1), false);
    assert_eq!(Solution::check_overlap(1, 0, 0, -1, 0, 0, 1), true)
}

struct Solution;
impl Solution {
    pub fn check_overlap(
        radius: i32,
        x_center: i32,
        y_center: i32,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
    ) -> bool {
        let x_closest = if x_center < x1 {
            x1
        } else if x_center > x2 {
            x2
        } else {
            x_center
        };
        let y_closest = if y_center < y1 {
            y1
        } else if y_center > y2 {
            y2
        } else {
            y_center
        };
        let x_distance = x_center - x_closest;
        let y_distance = y_center - y_closest;
        return x_distance * x_distance + y_distance * y_distance <= radius * radius;
    }
}
