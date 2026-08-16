fn main() {
    assert_eq!(Solution::car_fleet(12, vec![10,8,0,5,3], vec![2,4,1,1,3]), 3);
    assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    assert_eq!(Solution::car_fleet(100, vec![0,2,4], vec![4,2,1]), 1);
}

struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut cars: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
        cars.sort_unstable_by(|a, b| b.0.cmp(&a.0));

        let mut fleets = 0;
        let mut slowest_time_ahead = 0.0_f64;

        for (pos, spd) in cars {
            let time = (target - pos) as f64 / spd as f64;
            if time > slowest_time_ahead {
                fleets += 1;
                slowest_time_ahead = time;
            }
        }

        fleets
    }
}
