fn main() {
    assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 6.0), 1);
    assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 2.7), 3);
    assert_eq!(Solution::min_speed_on_time(vec![1, 3, 2], 1.9), -1);
}

struct Solution;
impl Solution {
    fn can_reach_on_time(dist: &Vec<i32>, hour: f64, speed: i32) -> bool {
        let mut time = 0.0;
        for i in 0..dist.len() - 1 {
            time += (dist[i] as f64 / speed as f64).ceil();
        }
        time += dist[dist.len() - 1] as f64 / speed as f64;
        time <= hour
    }
    pub fn min_speed_on_time(dist: Vec<i32>, hour: f64) -> i32 {
        return if hour < dist.len() as f64 - 1.0 {
            -1
        } else {
            let mut left = 1;
            let mut right = 1_000_000_000;
            while left < right {
                let mid = (left + right) / 2;
                if Self::can_reach_on_time(&dist, hour, mid) {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            left
        };
    }
}
