fn main() {
    assert_eq!(
        Solution::minimum_time(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
            vec![1, 1, 5]
        ),
        vec![0, -1, 4]
    );
    assert_eq!(
        Solution::minimum_time(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
            vec![1, 3, 5]
        ),
        vec![0, 2, 3]
    );
    assert_eq!(
        Solution::minimum_time(2, vec![vec![0, 1, 1]], vec![1, 1]),
        vec![0, -1]
    );
}

struct Solution;
impl Solution {
    pub fn minimum_time(n: i32, edges: Vec<Vec<i32>>, disappear: Vec<i32>) -> Vec<i32> {
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
        heap.push((0, 0)); // (time, node)
        while let Some((time, u)) = heap.pop() {
            let time = -time;
            if dist[u] < time {
                continue;
            }
            for &(v, w) in &graph[u] {
                let mut arrival = time + w;
                if arrival >= disappear[v] {
                    continue;
                }
                if arrival % 2 != 0 {
                    arrival += 1; // wait for even time
                }
                if arrival < dist[v] {
                    dist[v] = arrival;
                    heap.push((-arrival, v));
                }
            }
        }
        return dist;
    }
}
