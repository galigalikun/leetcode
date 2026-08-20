use std::collections::BinaryHeap;

fn main() {
    assert_eq!(Solution::min_refuel_stops(1, 1, vec![]), 0);
    assert_eq!(Solution::min_refuel_stops(100, 1, vec![vec![10, 100]]), -1);
    assert_eq!(
        Solution::min_refuel_stops(
            100,
            10,
            vec![vec![10, 60], vec![20, 30], vec![30, 30], vec![60, 40]]
        ),
        2
    );
}

struct Solution;
impl Solution {
    pub fn min_refuel_stops(target: i32, start_fuel: i32, stations: Vec<Vec<i32>>) -> i32 {
        let mut max_heap = BinaryHeap::new();
        let mut station_index = 0usize;
        let mut fuel = i64::from(start_fuel);
        let target_distance = i64::from(target);
        let mut stops = 0;

        while fuel < target_distance {
            while station_index < stations.len() && i64::from(stations[station_index][0]) <= fuel {
                max_heap.push(stations[station_index][1]);
                station_index += 1;
            }

            match max_heap.pop() {
                Some(next_fuel) => {
                    fuel += i64::from(next_fuel);
                    stops += 1;
                }
                None => return -1,
            }
        }

        stops
    }
}
