fn main() {
    assert_eq!(
        Solution::finding_users_active_minutes(
            vec![vec![0, 5], vec![1, 2], vec![0, 2], vec![0, 5], vec![1, 3]],
            5
        ),
        vec![0, 2, 0, 0, 0]
    );
    assert_eq!(
        Solution::finding_users_active_minutes(vec![vec![1, 1], vec![2, 2], vec![2, 3]], 4),
        vec![1, 1, 0, 0]
    );
}

struct Solution;
impl Solution {
    pub fn finding_users_active_minutes(logs: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut user_map = std::collections::HashMap::new();
        for log in logs {
            let user = log[0];
            let time = log[1];
            user_map
                .entry(user)
                .or_insert(std::collections::HashSet::new())
                .insert(time);
        }
        let mut count_map = vec![0; k as usize];
        for times in user_map.values() {
            let count = times.len();
            if count > 0 && count <= k as usize {
                count_map[count - 1] += 1;
            }
        }
        for i in 0..k as usize {
            if count_map[i] == 0 {
                count_map[i] = 0;
            }
        }
        count_map
    }
}
