fn main() {
    assert_eq!(
        Solution::is_escape_possible(vec![vec![0, 1], vec![1, 0]], vec![0, 0], vec![0, 2]),
        false
    );
    assert_eq!(
        Solution::is_escape_possible(vec![], vec![0, 0], vec![999999, 999999]),
        true
    );
}

struct Solution;
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let mut blocked_set = std::collections::HashSet::new();
        for b in blocked {
            blocked_set.insert((b[0], b[1]));
        }
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((source[0], source[1]));
        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();
            if x == target[0] && y == target[1] {
                return true;
            }
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            if x > 0 && !blocked_set.contains(&(x - 1, y)) {
                queue.push_back((x - 1, y));
            }
            if x < 999999 && !blocked_set.contains(&(x + 1, y)) {
                queue.push_back((x + 1, y));
            }
            if y > 0 && !blocked_set.contains(&(x, y - 1)) {
                queue.push_back((x, y - 1));
            }
            if y < 999999 && !blocked_set.contains(&(x, y + 1)) {
                queue.push_back((x, y + 1));
            }
        }
        return false;
    }
}
