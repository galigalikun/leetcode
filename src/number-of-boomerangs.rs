fn main() {
    assert_eq!(
        Solution::number_of_boomerangs(vec![vec![0, 0], vec![1, 0], vec![2, 0]]),
        2
    );
    assert_eq!(
        Solution::number_of_boomerangs(vec![vec![1, 1], vec![2, 2], vec![3, 3]]),
        2
    );
    assert_eq!(Solution::number_of_boomerangs(vec![vec![1, 1]]), 0);
}

pub struct Solution {}
// https://github.com/cherryljr/LeetCode/blob/master/Number%20of%20Boomerangs.java
use std::collections::HashMap;
impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        for i in 0..points.len() {
            let mut map = HashMap::new();
            for j in 0..points.len() {
                if i == j {
                    continue;
                }
                let distance = (points[i][0] - points[j][0], points[i][1] - points[j][1]);
                let key = distance.0 * distance.0 + distance.1 * distance.1;
                if let Some(m) = map.get_mut(&key) {
                    *m += 1;
                } else {
                    map.insert(key, 1);
                }
            }
            for (_, v) in map {
                result += v * (v - 1);
            }
        }
        return result;
    }
}
