fn main() {
    assert_eq!(
        Solution::watched_videos_by_friends(
            vec![
                vec!["A".to_string(), "B".to_string()],
                vec!["C".to_string()],
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string()]
            ],
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            0,
            1
        ),
        vec!["B", "C"]
    );
    assert_eq!(
        Solution::watched_videos_by_friends(
            vec![
                vec!["A".to_string(), "B".to_string()],
                vec!["C".to_string()],
                vec!["B".to_string(), "C".to_string()],
                vec!["D".to_string()]
            ],
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            0,
            2
        ),
        vec!["D"]
    );
}

struct Solution;
impl Solution {
    pub fn watched_videos_by_friends(
        watched_videos: Vec<Vec<String>>,
        friends: Vec<Vec<i32>>,
        id: i32,
        level: i32,
    ) -> Vec<String> {
        let mut queue = vec![id];
        let mut visited = vec![false; friends.len()];
        visited[id as usize] = true;
        let mut level = level;
        while level > 0 {
            let mut next_queue = vec![];
            for i in queue {
                for j in &friends[i as usize] {
                    if !visited[*j as usize] {
                        visited[*j as usize] = true;
                        next_queue.push(*j);
                    }
                }
            }
            queue = next_queue;
            level -= 1;
        }
        let mut map = std::collections::HashMap::new();
        for i in queue {
            for j in &watched_videos[i as usize] {
                *map.entry(j).or_insert(0) += 1;
            }
        }
        let mut res = vec![];
        for (k, _v) in map {
            res.push(k);
        }
        res.sort_by(|a, b| a.cmp(b));
        return res.iter().map(|x| x.to_string()).collect();
    }
}
