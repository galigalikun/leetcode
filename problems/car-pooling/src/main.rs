fn main() {
    assert_eq!(
        Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 4),
        false
    );
    assert_eq!(
        Solution::car_pooling(vec![vec![2, 1, 5], vec![3, 3, 7]], 5),
        true
    );
}

struct Solution;
impl Solution {
    pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
        let mut stops = [0; 1001];
        for trip in trips {
            stops[trip[1] as usize] += trip[0];
            stops[trip[2] as usize] -= trip[0];
        }
        let mut capacity = capacity;
        for stop in stops.iter() {
            capacity -= stop;
            if capacity < 0 {
                return false;
            }
        }

        return true;
    }
}
