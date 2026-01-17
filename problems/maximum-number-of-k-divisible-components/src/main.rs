fn main() {
    assert_eq!(
        Solution::max_k_divisible_components(
            5,
            vec![[0,2],[1,2],[1,3],[2,4]].iter().map(|e| e.to_vec()).collect(),
            vec![1,8,1,4,4],
            6
        ),
        2
    );
    assert_eq!(
        Solution::max_k_divisible_components(
            7,
            vec![[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]]
                .iter()
                .map(|e| e.to_vec())
                .collect(),
            vec![3,0,6,1,5,2,1],
            3
        ),
        3
    );
}

struct Solution;
impl Solution {
    pub fn max_k_divisible_components(n: i32, edges: Vec<Vec<i32>>, values: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        for edge in edges.iter() {
            graph.entry(edge[0]).or_default().push(edge[1]);
            graph.entry(edge[1]).or_default().push(edge[0]);
        }

        let mut visited = vec![false; n as usize + 1];
        let mut result = 0;

        fn dfs(
            node: i32,
            graph: &HashMap<i32, Vec<i32>>,
            values: &Vec<i32>,
            visited: &mut Vec<bool>,
            k: i32,
            result: &mut i32,
        ) -> i32 {
            visited[node as usize] = true;
            let mut total = values[(node - 1) as usize];

            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors.iter() {
                    if !visited[neighbor as usize] {
                        total += dfs(neighbor, graph, values, visited, k, result);
                    }
                }
            }

            if total % k == 0 {
                *result += 1;
                return 0;
            }

            total
        }

        for node in 1..=n {
            if !visited[node as usize] {
                dfs(node, &graph, &values, &mut visited, k, &mut result);
            }
        }

        result
    }
}
