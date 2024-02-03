fn main() {
    assert_eq!(
        Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        3
    );
    assert_eq!(
        Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut days = vec![0; 100001];
        let mut count = 0;
        for event in events {
            for i in event[0]..event[1] + 1 {
                if days[i as usize] == 0 {
                    days[i as usize] = 1;
                    count += 1;
                    break;
                }
            }
        }
        return count;
    }
}
