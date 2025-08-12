fn main() {
    assert_eq!(
        Solution::split_painting(vec![vec![1, 4, 5], vec![4, 7, 7], vec![1, 7, 9]]),
        vec![vec![1, 4, 14], vec![4, 7, 16]]
    );
    assert_eq!(
        Solution::split_painting(vec![vec![1, 7, 9], vec![6, 8, 15], vec![8, 10, 7]]),
        vec![vec![1, 6, 9], vec![6, 7, 24], vec![8, 10, 7]]
    );
    assert_eq!(
        Solution::split_painting(vec![
            vec![1, 4, 5],
            vec![1, 4, 7],
            vec![4, 7, 1],
            vec![4, 7, 11]
        ]),
        vec![vec![1, 4, 12], vec![4, 7, 12]]
    );
}

struct Solution;
impl Solution {
    pub fn split_painting(segments: Vec<Vec<i32>>) -> Vec<Vec<i64>> {
        let mut events = vec![];
        for segment in segments {
            events.push((segment[0] as i64, segment[2] as i64));
            events.push((segment[1] as i64, -(segment[2] as i64)));
        }
        events.sort_unstable();
        let mut result = vec![];
        let mut current_color: i64 = 0;
        let mut last_position: i64 = -1;
        for (position, color) in events {
            if current_color != 0 && last_position != -1 && position != last_position {
                result.push(vec![last_position, position, current_color]);
            }
            current_color += color;
            last_position = position;
        }
        result
    }
}
