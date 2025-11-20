fn main() {
    assert_eq!(
        Solution::find_answer(
            6,
            vec![
                vec![0, 1, 4],
                vec![0, 2, 1],
                vec![1, 3, 2],
                vec![1, 4, 3],
                vec![1, 5, 1],
                vec![2, 3, 1],
                vec![3, 5, 3],
                vec![4, 5, 2]
            ]
        ),
        vec![true, true, true, false, true, true, true, false]
    );
    assert_eq!(
        Solution::find_answer(
            4,
            vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]]
        ),
        vec![true, false, false, true]
    );
}

struct Solution;
impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        let mut dist = vec![i32::MAX; n as usize];
        dist[0] = 0;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((0, 0)); // (distance, node)
        while let Some((d, u)) = heap.pop() {
            let d = -d;
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                let nd = d + w;
                if nd < dist[v] {
                    dist[v] = nd;
                    heap.push((-nd, v));
                }
            }
        }
        dist.iter().map(|&d| d % 2 == 0).collect()
    }
}
