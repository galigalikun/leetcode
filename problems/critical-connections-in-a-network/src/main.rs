fn main() {
    assert_eq!(
        Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]),
        vec![[1, 3]]
    );
    assert_eq!(
        Solution::critical_connections(2, vec![vec![0, 1]]),
        vec![[0, 1]]
    );
}

struct Solution;
impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for conn in connections.iter() {
            graph[conn[0] as usize].push(conn[1] as usize);
            graph[conn[1] as usize].push(conn[0] as usize);
        }

        let mut visited = vec![false; n];
        let mut disc = vec![0; n];
        let mut low = vec![0; n];
        let mut bridges = vec![];
        let mut time = vec![0];

        for i in 0..n {
            if !visited[i] {
                Self::dfs(
                    i,
                    usize::MAX,
                    &graph,
                    &mut visited,
                    &mut disc,
                    &mut low,
                    &mut time,
                    &mut bridges,
                );
            }
        }

        bridges
    }

    fn dfs(
        u: usize,
        parent: usize,
        graph: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        disc: &mut Vec<usize>,
        low: &mut Vec<usize>,
        time: &mut Vec<usize>,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        visited[u] = true;
        disc[u] = time[0];
        low[u] = time[0];
        time[0] += 1;

        for &v in &graph[u] {
            if !visited[v] {
                Self::dfs(v, u, graph, visited, disc, low, time, bridges);

                low[u] = low[u].min(low[v]);

                if low[v] > disc[u] {
                    bridges.push(vec![u as i32, v as i32]);
                }
            } else if v != parent {
                low[u] = low[u].min(disc[v]);
            }
        }
    }
}
