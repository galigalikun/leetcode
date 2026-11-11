fn main() {
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
        vec![0, 1, -1]
    );
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
        vec![0, 1, -1]
    );
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![vec![0, 2]]),
        vec![0, 1, 1]
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

        const INF: i32 = i32::MAX / 2;
        let mut dist = vec![vec![INF; 2]; n as usize];
        let mut queue = std::collections::VecDeque::new();

        for color in 0..2 {
            dist[0][color] = 0;
            queue.push_back((0, color));
        }

        while let Some((node, prev_color)) = queue.pop_front() {
            for &(next, edge_color) in &graph[node] {
                if edge_color == prev_color {
                    continue;
                }

                let next_dist = dist[node][prev_color] + 1;
                if next_dist < dist[next][edge_color] {
                    dist[next][edge_color] = next_dist;
                    queue.push_back((next, edge_color));
                }
            }
        }

        let mut ans = vec![-1; n as usize];
        for (idx, states) in dist.iter().enumerate() {
            let min_dist = states[0].min(states[1]);
            if min_dist != INF {
                ans[idx] = min_dist;
            }
        }

        ans
    }
}
