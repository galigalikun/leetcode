fn main() {
    assert_eq!(
        Solution::count_paths(
            7,
            vec![
                vec![0, 6, 7],
                vec![0, 1, 2],
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![6, 3, 3],
                vec![3, 5, 1],
                vec![6, 5, 1],
                vec![2, 5, 1],
                vec![0, 4, 5],
                vec![4, 6, 2]
            ]
        ),
        4
    );
    assert_eq!(Solution::count_paths(2, vec![vec![1, 0, 10],]), 1);
}

struct Solution;
impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut g = vec![vec![]; n];
        for r in roads {
            let (a, b, c) = (r[0] as usize, r[1] as usize, r[2] as i32);
            g[a].push((b, c));
            g[b].push((a, c));
        }
        let mut dist = vec![i32::MAX; n];
        dist[0] = 0;
        let mut ways = vec![0; n];
        ways[0] = 1;
        let mut heap = std::collections::BinaryHeap::new();
        heap.push((0, 0)); // (cost, node)
        while let Some((cost, u)) = heap.pop() {
            let cost = -cost;
            if cost > dist[u] {
                continue;
            }
            for &(v, w) in &g[u] {
                let next = cost + w;
                if next < dist[v] {
                    dist[v] = next;
                    ways[v] = ways[u];
                    heap.push((-next, v));
                } else if next == dist[v] {
                    ways[v] = (ways[v] + ways[u]) % 1_000_000_007;
                }
            }
        }
        ways[n - 1]
    }
}
