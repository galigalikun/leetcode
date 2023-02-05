use std::collections::HashMap;

fn main() {
    assert_eq!(
        Solution::min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![2, 2]
        ]),
        4
    );
    assert_eq!(
        Solution::min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3]
        ]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let mut x_map = HashMap::new();
        let mut y_map = HashMap::new();
        let mut x_ans = std::i32::MAX;
        let mut y_ans = std::i32::MAX;
        for point in points {
            for (x, y_vec) in x_map.iter() {
                if point[0] == *x {
                    for y in y_vec {
                        y_ans = std::cmp::min(y_ans, y - point[1]);
                    }
                }
            }
            for (y, x_vec) in y_map.iter() {
                if point[1] == *y {
                    for x in x_vec {
                        x_ans = std::cmp::min(x_ans, x - point[0]);
                    }
                }
            }
            (*x_map.entry(point[0]).or_insert(vec![point[1]])).push(point[1]);
            (*y_map.entry(point[1]).or_insert(vec![point[0]])).push(point[0]);
        }
        return x_ans * y_ans;
    }
}
