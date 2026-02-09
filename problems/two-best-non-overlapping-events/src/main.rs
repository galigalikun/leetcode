fn main() {
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![2, 4, 3]]),
        4
    );
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 3, 2], vec![4, 5, 2], vec![1, 5, 5]]),
        5
    );
    assert_eq!(
        Solution::max_two_events(vec![vec![1, 5, 3], vec![1, 5, 1], vec![6, 6, 5]]),
        8
    );
}

struct Solution;
impl Solution {
    pub fn max_two_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by_key(|e| e[0]);
        let n = events.len();
        let mut max_values = vec![0; n + 1];
        for i in (0..n).rev() {
            max_values[i] = max_values[i + 1].max(events[i][2]);
        }
        let mut result = 0;
        for i in 0..n {
            let mut left = i + 1;
            let mut right = n;
            while left < right {
                let mid = (left + right) / 2;
                if events[mid][0] > events[i][1] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
            result = result.max(events[i][2] + max_values[left]);
        }
        return result;
    }
}
