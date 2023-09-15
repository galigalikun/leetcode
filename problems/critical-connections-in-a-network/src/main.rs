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
        let mut graph = vec![vec![]; n as usize];
        for conn in connections.iter() {
            graph[conn[0] as usize].push(conn[1]);
            graph[conn[1] as usize].push(conn[0]);
        }
        return graph;
    }
}
