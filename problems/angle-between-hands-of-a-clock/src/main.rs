fn main() {
    assert_eq!(Solution::angle_clock(12, 30), 165.0);
    assert_eq!(Solution::angle_clock(3, 30), 75.0);
    assert_eq!(Solution::angle_clock(3, 15), 7.5);
}

struct Solution;
impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let hour = hour as f64;
        let minutes = minutes as f64;
        let hour_angle = (hour * 30.0) + (minutes * 0.5);
        let minutes_angle = minutes * 6.0;
        let angle = (hour_angle - minutes_angle).abs();
        return if angle <= 180.0 { angle } else { 360.0 - angle };
    }
}
