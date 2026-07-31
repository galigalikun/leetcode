fn main() {
    assert_eq!(Solution::num_buses_to_destination(vec![vec![1,2,7],vec![3,6,7]], 1, 6), 2);
    assert_eq!(Solution::num_buses_to_destination(vec![vec![7,12],vec![4,5,15],vec![6],vec![15,19],vec![9,12,13]], 15, 12), -1);
}

use std::collections::{HashMap, HashSet, VecDeque};

struct Solution{}
impl Solution {
    pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
        if source == target {
            return 0;
        }

        let mut stop_to_buses: HashMap<i32, Vec<usize>> = HashMap::new();
        for (bus_id, route) in routes.iter().enumerate() {
            for &stop in route {
                stop_to_buses.entry(stop).or_default().push(bus_id);
            }
        }

        let mut visited_stops: HashSet<i32> = HashSet::new();
        let mut taken_buses = vec![false; routes.len()];
        let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

        visited_stops.insert(source);
        queue.push_back((source, 0));

        while let Some((stop, buses_taken)) = queue.pop_front() {
            if let Some(buses) = stop_to_buses.get(&stop) {
                for &bus_id in buses {
                    if taken_buses[bus_id] {
                        continue;
                    }
                    taken_buses[bus_id] = true;

                    for &next_stop in &routes[bus_id] {
                        if next_stop == target {
                            return buses_taken + 1;
                        }
                        if visited_stops.insert(next_stop) {
                            queue.push_back((next_stop, buses_taken + 1));
                        }
                    }
                }
            }
        }

        -1
    }
}
