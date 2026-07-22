fn main() {
    assert_eq!(
        Solution::all_paths_source_target(vec![vec![1, 2], vec![3], vec![3], vec![]]),
        vec![vec![0, 1, 3], vec![0, 2, 3]]
    );
    assert_eq!(
        Solution::all_paths_source_target(vec![
            vec![4, 3, 1],
            vec![3, 2, 4],
            vec![3],
            vec![4],
            vec![]
        ]),
        vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4]
        ]
    );
}

struct Solution {}
impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let target = (graph.len() - 1) as i32;
        let mut paths = Vec::new();
        let mut path = vec![0];

        Self::dfs(0, target, &graph, &mut path, &mut paths);

        paths
    }

    fn dfs(
        node: i32,
        target: i32,
        graph: &[Vec<i32>],
        path: &mut Vec<i32>,
        paths: &mut Vec<Vec<i32>>,
    ) {
        if node == target {
            paths.push(path.clone());
            return;
        }

        for &next in &graph[node as usize] {
            path.push(next);
            Self::dfs(next, target, graph, path, paths);
            path.pop();
        }
    }
}
