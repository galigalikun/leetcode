fn main() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2),
        8
    );
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0),
        6
    );
    assert_eq!(
        Solution::least_interval(
            vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'],
            2
        ),
        16
    );
}

// https://medium.com/@swgarciab/task-scheduler-leetcode-problem-a74acadf0e22
struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        if n == 0 {
            return tasks.len() as i32;
        }
        let mut map = HashMap::new();
        for task in tasks.clone() {
            if let Some(m) = map.get_mut(&task) {
                *m += 1;
            } else {
                map.insert(task, 1);
            }
        }
        let max = map.values().max().unwrap();
        let last_length = map.values().filter(|&x| x == max).count();
        return std::cmp::max(tasks.len(), (max - 1) * (n as usize + 1) + last_length) as i32;
    }
}
