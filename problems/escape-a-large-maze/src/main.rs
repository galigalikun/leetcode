use std::collections::{HashSet, VecDeque};

const MAX_COORD: i32 = 1_000_000;

fn main() {
    assert_eq!(
        Solution::is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]),
        false
    );
    assert_eq!(
        Solution::is_escape_possible(vec![], vec![0, 0], vec![999999, 999999]),
        true
    );
    assert_eq!(
        Solution::is_escape_possible(
            vec![vec![0, 0], vec![0, 1], vec![1, 1], vec![1, 0]],
            vec![0, 2],
            vec![2, 2]
        ),
        true
    );
    assert_eq!(
        Solution::is_escape_possible(
            vec![vec![2, 2], vec![2, 3], vec![3, 2], vec![3, 3]],
            vec![0, 0],
            vec![1, 1]
        ),
        true
    );
}

struct Solution;
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        if source == target {
            return true;
        }

        let mut blocked_set = HashSet::new();
        for b in blocked {
            blocked_set.insert((b[0], b[1]));
        }

        if blocked_set.contains(&(source[0], source[1]))
            || blocked_set.contains(&(target[0], target[1]))
        {
            return false;
        }

        if blocked_set.is_empty() {
            return true;
        }

        let limit = blocked_set.len() * blocked_set.len();

        let (source_found, source_size) = Self::reachable(&blocked_set, (source[0], source[1]), (target[0], target[1]), limit);
        if source_found {
            return true;
        }

        let (target_found, target_size) = Self::reachable(&blocked_set, (target[0], target[1]), (source[0], source[1]), limit);
        if target_found {
            return true;
        }

        source_size > limit && target_size > limit
    }

    fn reachable(
        blocked: &HashSet<(i32, i32)>,
        start: (i32, i32),
        target: (i32, i32),
        limit: usize,
    ) -> (bool, usize) {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(start);
        seen.insert(start);

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            if (x, y) == target {
                return (true, seen.len());
            }

            if seen.len() > limit {
                return (false, seen.len());
            }

            for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || nx >= MAX_COORD || ny < 0 || ny >= MAX_COORD {
                    continue;
                }

                let next = (nx, ny);
                if blocked.contains(&next) || seen.contains(&next) {
                    continue;
                }

                seen.insert(next);
                queue.push_back(next);
            }
        }

        (false, seen.len())
    }
}
