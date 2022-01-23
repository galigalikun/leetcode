fn main() {
    assert_eq!(
        Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 1]),
        true
    );
    assert_eq!(
        Solution::valid_square(vec![0, 0], vec![1, 1], vec![1, 0], vec![0, 12]),
        false
    );
    assert_eq!(
        Solution::valid_square(vec![1, 0], vec![-1, 0], vec![0, 1], vec![0, -1]),
        true
    );
    assert_eq!(
        Solution::valid_square(vec![1, 1], vec![0, 1], vec![1, 2], vec![0, 0]),
        false
    );
}

// https://www.geeksforgeeks.org/check-given-four-points-form-square/
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        let v = vec![p1, p2, p3, p4];
        let mut map = HashMap::new();
        for i in 0..4 {
            for j in i + 1..4 {
                let (x1, y1, x2, y2) = (v[i][0], v[i][1], v[j][0], v[j][1]);
                let dist = (x1 - x2).pow(2) + (y1 - y2).pow(2);
                if dist == 0 {
                    return false;
                }
                if let Some(m) = map.get_mut(&dist) {
                    *m += 1;
                } else {
                    map.insert(dist, 1);
                }
            }
        }

        return map.len() == 2;
    }
}
