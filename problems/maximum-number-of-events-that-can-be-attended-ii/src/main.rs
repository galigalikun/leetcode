fn main() {
    assert_eq!(
        Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2),
        7
    );
    assert_eq!(
        Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 10]], 2),
        10
    );
    assert_eq!(
        Solution::max_value(
            vec![vec![1, 1, 1], vec![2, 2, 2], vec![3, 3, 3], vec![4, 4, 4]],
            3
        ),
        9
    );
    assert_eq!(
        Solution::max_value(vec![vec![30, 40, 34], vec![6, 11, 6], vec![60, 81, 36]], 1),
        7
    );
    assert_eq!(
        Solution::max_value(vec![vec![1, 2, 4], vec![3, 4, 3], vec![2, 3, 1]], 2),
        7
    );
}

struct Solution;
impl Solution {
    pub fn max_value(events: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut events = events;
        events.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut dp = vec![0; k as usize + 1];
        for _i in 1..=k as usize {
            let mut dp2 = vec![0; events.len() + 1];
            let mut j = 0;
            for (idx, event) in events.iter().enumerate() {
                while j < events.len() && events[j][1] < event[0] {
                    j += 1;
                }
                if dp[j] == 0 {
                    break;
                }
                dp2[idx + 1] = dp2[idx].max(dp[j] + event[2]);
            }
            dp = dp2;
        }
        dp[events.len()]
    }
}
