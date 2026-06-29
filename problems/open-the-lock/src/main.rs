fn main() {
    assert_eq!(
        Solution::open_lock(
            vec![
                "0201".to_string(),
                "0101".to_string(),
                "0102".to_string(),
                "1212".to_string(),
                "2002".to_string()
            ],
            "0202".to_string()
        ),
        6
    );
    assert_eq!(
        Solution::open_lock(vec!["8888".to_string()], "0009".to_string()),
        1
    );
    assert_eq!(
        Solution::open_lock(
            vec![
                "8887".to_string(),
                "8889".to_string(),
                "8878".to_string(),
                "8898".to_string(),
                "8788".to_string(),
                "8988".to_string(),
                "7888".to_string(),
                "9888".to_string()
            ],
            "8888".to_string()
        ),
        -1
    );
}

use std::collections::HashSet;
use std::collections::VecDeque;

struct Solution {}
impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<String> = deadends.into_iter().collect();

        let start = "0000".to_string();
        if dead.contains(&start) {
            return -1;
        }
        if target == start {
            return 0;
        }

        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.clone());

        let mut queue: VecDeque<(String, i32)> = VecDeque::new();
        queue.push_back((start, 0));

        while let Some((state, turns)) = queue.pop_front() {
            for next in Self::neighbors(&state) {
                if dead.contains(&next) || visited.contains(&next) {
                    continue;
                }
                if next == target {
                    return turns + 1;
                }
                visited.insert(next.clone());
                queue.push_back((next, turns + 1));
            }
        }

        -1
    }

    /// Returns all states reachable by turning a single wheel one slot.
    fn neighbors(state: &str) -> Vec<String> {
        let digits: Vec<u8> = state.bytes().map(|b| b - b'0').collect();
        let mut result = Vec::with_capacity(8);

        for i in 0..digits.len() {
            for delta in [1u8, 9u8] {
                let mut next = digits.clone();
                next[i] = (next[i] + delta) % 10;
                let s: String = next.iter().map(|d| (d + b'0') as char).collect();
                result.push(s);
            }
        }

        result
    }
}
