fn main() {
    assert_eq!(
        Solution::max_stability(
            3,
            vec![[0, 1, 2, 1], [1, 2, 3, 0]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            1
        ),
        2
    );
    assert_eq!(
        Solution::max_stability(
            3,
            vec![[0, 1, 4, 0], [1, 2, 3, 0], [0, 2, 1, 0]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            2
        ),
        6
    );
    assert_eq!(
        Solution::max_stability(
            3,
            vec![[0, 1, 1, 1], [1, 2, 1, 1], [2, 0, 1, 1]]
                .iter()
                .map(|v| v.to_vec())
                .collect(),
            0
        ),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        for neighbors in graph.iter_mut() {
            neighbors.sort_unstable();
        }
        let mut visited = vec![false; n as usize];
        let mut stack = vec![(0, 0)];
        let mut stability = 0;
        while let Some((node, depth)) = stack.pop() {
            if visited[node as usize] {
                continue;
            }
            visited[node as usize] = true;
            if depth == k {
                stability += 1;
            }
            for &neighbor in &graph[node as usize] {
                if !visited[neighbor as usize] {
                    stack.push((neighbor, depth + 1));
                }
            }
        }
        if stability == 0 { -1 } else { stability }
    }
}
