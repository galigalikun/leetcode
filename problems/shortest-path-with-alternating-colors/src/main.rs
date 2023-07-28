fn main() {
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
        vec![0, 1, -1]
    );
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
        vec![0, 1, -1]
    );
}

struct Solution;
impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let mut graph = vec![vec![]; n as usize];
        for edge in red_edges {
            graph[edge[0] as usize].push((edge[1] as usize, 0));
        }
        for edge in blue_edges {
            graph[edge[0] as usize].push((edge[1] as usize, 1));
        }
        let mut ans = vec![-1; n as usize];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 0));
        queue.push_back((0, 0, 1));
        while let Some((node, dist, color)) = queue.pop_front() {
            if ans[node] != -1 {
                continue;
            }
            ans[node] = dist;
            for &(next, next_color) in graph[node].iter() {
                if next_color != color {
                    queue.push_back((next, dist + 1, next_color));
                }
            }
        }
        return ans;
    }
}
