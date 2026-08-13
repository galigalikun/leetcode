use std::collections::VecDeque;

fn main() {
    assert_eq!(Solution::shortest_path_length(vec![vec![1,2,3],vec![0],vec![0],vec![0]]), 4);
    assert_eq!(Solution::shortest_path_length(vec![vec![1],vec![0,2,4],vec![1,3,4],vec![2],vec![1,2]]), 4);
}

struct Solution;
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let n = graph.len();
        if n <= 1 {
            return 0;
        }

        let target_mask = (1usize << n) - 1;
        let mut queue: VecDeque<(usize, usize, i32)> = VecDeque::new();
        let mut seen = vec![vec![false; 1usize << n]; n];

        for node in 0..n {
            let mask = 1usize << node;
            queue.push_back((node, mask, 0));
            seen[node][mask] = true;
        }

        while let Some((node, mask, dist)) = queue.pop_front() {
            if mask == target_mask {
                return dist;
            }

            for &next in &graph[node] {
                let next_node = next as usize;
                let next_mask = mask | (1usize << next_node);
                if seen[next_node][next_mask] {
                    continue;
                }
                seen[next_node][next_mask] = true;
                queue.push_back((next_node, next_mask, dist + 1));
            }
        }

        0
    }
}
