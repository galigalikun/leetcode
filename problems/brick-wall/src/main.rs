fn main() {
    assert_eq!(Solution::least_bricks(vec![vec![1,2,2,1],vec![3,1,2],vec![1,3,2],vec![2,4],vec![3,1,2],vec![1,3,1,1]]), 2);
    assert_eq!(Solution::least_bricks(vec![vec![1],vec![1],vec![1]]), 3);
}

// https://dilyar85.gitbooks.io/leetcode-solutions-with-analysis/content/Problems/554_brick_wall_java.html
struct Solution{}
use std::collections::HashMap;
impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let mut map = HashMap::new();
        let mut count = 0;
        for mut w in wall.clone() {
            let mut end = 0;
            w.pop();
            for p in w {
                end += p;
                if let Some(m) = map.get_mut(&end) {
                    *m += 1;
                } else {
                    map.insert(end, 1);
                }
                count = std::cmp::max(count, *map.get(&end).unwrap());
            }
        }
        return wall.len() as i32 - count;
    }
}
