fn main() {
    assert_eq!(Solution::max_probability(3, vec![vec![0,1],vec![1,2],vec![0,2]], vec![0.5,0.5,0.2], 0, 2), 0.25);
    assert_eq!(Solution::max_probability(3, vec![vec![0,1]], vec![0.5], 0, 2), 0.0);
    assert_eq!(Solution::max_probability(3, vec![vec![0,1],vec![1,2],vec![0,2]], vec![0.5,0.5,0.3], 0, 2), 0.3);
}

struct Solution;
impl Solution {
    pub fn max_probability(n: i32, edges: Vec<Vec<i32>>, succ_prob: Vec<f64>, start_node: i32, end_node: i32) -> f64 {
        use std::collections::HashMap;
        use std::collections::BinaryHeap;
        let mut graph = HashMap::new();
        for i in 0..edges.len() {
            let edge = &edges[i];
            let prob = succ_prob[i];
            graph.entry(edge[0]).or_insert(Vec::new()).push((edge[1], prob));
            graph.entry(edge[1]).or_insert(Vec::new()).push((edge[0], prob));
        }
        let mut heap = BinaryHeap::new();
        heap.push((0.0, start_node));
        let mut visited = vec![false; n as usize];
        let mut probs = vec![0.0; n as usize];
        probs[start_node as usize] = 1.0;
        while let Some((prob, node)) = heap.pop() {
            if visited[node as usize] {
                continue;
            }
            visited[node as usize] = true;
            if node == end_node {
                return prob;
            }
            if let Some(neighbors) = graph.get(&node) {
                for (neighbor, edge_prob) in neighbors {
                    let new_prob = prob * edge_prob;
                    if new_prob > probs[*neighbor as usize] {
                        probs[*neighbor as usize] = new_prob;
                        heap.push((new_prob, *neighbor));
                    }
                }
            }
        }
        return 0.0;
    }
}
