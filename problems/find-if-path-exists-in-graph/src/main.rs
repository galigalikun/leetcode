fn main() {
    assert_eq!(
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 2),
        true
    );
    assert_eq!(
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 1),
        true
    );
    assert_eq!(
        Solution::valid_path(3, vec![vec![0, 1], vec![1, 2], vec![2, 0]], 0, 3),
        false
    );
}

struct Solution;
impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut graph = vec![Vec::new(); n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }

        let mut visited = std::collections::HashSet::new();
        let mut stack = vec![source];
        while let Some(node) = stack.pop() {
            if node == destination {
                return true;
            }
            if visited.insert(node) {
                for &neighbor in &graph[node as usize] {
                    if !visited.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        false
    }
}
