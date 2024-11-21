fn main() {
    assert_eq!(Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9), 3);
    assert_eq!(
        Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11),
        -1
    );
    assert_eq!(
        Solution::minimum_jumps(vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7),
        2
    );
}

struct Solution;
impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        let forbidden = forbidden
            .into_iter()
            .collect::<std::collections::HashSet<_>>();
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, false));
        visited.insert((0, false));
        while let Some((pos, steps, back)) = queue.pop_front() {
            if pos == x {
                return steps;
            }
            if pos + a < 6000
                && !forbidden.contains(&(pos + a))
                && !visited.contains(&(pos + a, false))
            {
                queue.push_back((pos + a, steps + 1, false));
                visited.insert((pos + a, false));
            }
            if !back
                && pos - b > 0
                && !forbidden.contains(&(pos - b))
                && !visited.contains(&(pos - b, true))
            {
                queue.push_back((pos - b, steps + 1, true));
                visited.insert((pos - b, true));
            }
        }
        -1
    }
}
