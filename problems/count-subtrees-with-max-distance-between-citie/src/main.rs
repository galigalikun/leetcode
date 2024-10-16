fn main() {
    assert_eq!(Solution::count_subgraphs_for_each_diameter(4, vec![vec![1,2],vec![2,3],vec![2,4]]), vec![3,4,0]);
    assert_eq!(Solution::count_subgraphs_for_each_diameter(2, vec![vec![1,2]]), vec![1]);
    assert_eq!(Solution::count_subgraphs_for_each_diameter(3, vec![vec![1,2],vec![2,3]]), vec![2,1]);
}

struct Solution;
impl Solution {
    pub fn count_subgraphs_for_each_diameter(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![0; n as usize]; n as usize];
        for edge in edges.iter() {
            let (a, b) = (edge[0] as usize - 1, edge[1] as usize - 1);
            graph[a][b] = 1;
            graph[b][a] = 1;
        }
        let mut res = vec![0; n as usize - 1];
        for i in 1..(1 << n) {
            let mut nodes = vec![];
            for j in 0..n {
                if i & (1 << j) != 0 {
                    nodes.push(j);
                }
            }
            let mut g = vec![vec![0; n as usize]; n as usize];
            for &node in nodes.iter() {
                for &node2 in nodes.iter() {
                    g[node as usize][node2 as usize] = graph[node as usize][node2 as usize];
                }
            }
            let mut dist = vec![vec![n; n as usize]; n as usize];
            for &node in nodes.iter() {
                dist[node as usize][node as usize] = 0;
            }
            for &node in nodes.iter() {
                for &node2 in nodes.iter() {
                    if g[node as usize][node2 as usize] == 1 {
                        dist[node as usize][node2 as usize] = 1;
                    }
                }
            }
        }
        res
    }
}
