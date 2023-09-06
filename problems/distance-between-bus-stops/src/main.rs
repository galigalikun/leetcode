fn main() {
    assert_eq!(
        Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 1),
        1
    );
    assert_eq!(
        Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 2),
        3
    );
    assert_eq!(
        Solution::distance_between_bus_stops(vec![1, 2, 3, 4], 0, 3),
        4
    );
}

struct Solution;
impl Solution {
    pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
        let mut total = 0;
        let mut start = start as usize;
        let mut destination = destination as usize;
        if start > destination {
            std::mem::swap(&mut start, &mut destination);
        }
        for i in start..destination {
            total += distance[i];
        }
        let mut total2 = 0;
        for i in destination..distance.len() {
            total2 += distance[i];
        }
        for i in 0..start {
            total2 += distance[i];
        }
        if total < total2 {
            return total;
        }
        return total2;
    }
}
