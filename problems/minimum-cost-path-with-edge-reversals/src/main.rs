fn main() {
    assert_eq!(
        Solution::min_cost(
            4,
            vec![[0, 1, 3], [3, 1, 1], [2, 3, 4], [0, 2, 2]]
                .iter()
                .map(|x| x.to_vec())
                .collect()
        ),
        5
    );
    assert_eq!(
        Solution::min_cost(
            4,
            vec![[0, 2, 1], [2, 1, 1], [1, 3, 1], [2, 3, 3]]
                .iter()
                .map(|x| x.to_vec())
                .collect()
        ),
        3
    );
}

struct Solution;
impl Solution {
    fn dfs(graph: &Vec<Vec<(i32, i32)>>, node: i32, target: i32, visited: &mut Vec<bool>) -> i32 {
        if node == target {
            return 0;
        }
        visited[node as usize] = true;
        let mut ans = -1;
        for (neighbor, cost) in &graph[node as usize] {
            if !visited[*neighbor as usize] {
                let sub_cost = Self::dfs(graph, *neighbor, target, visited);
                if sub_cost != -1 {
                    if ans == -1 || ans > sub_cost + cost {
                        ans = sub_cost + cost;
                    }
                }
            }
        }
        visited[node as usize] = false;
        return ans;
    }
    pub fn min_cost(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push((edge[1], edge[2]));
        }
        return Self::dfs(&graph, 0, n - 1, &mut vec![false; n as usize]);
    }
}
