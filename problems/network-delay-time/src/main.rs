fn main() {
    assert_eq!(
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2),
        2
    );
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 1), 1);
    assert_eq!(Solution::network_delay_time(vec![vec![1, 2, 1]], 2, 2), -1);
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution {}
impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        // Build adjacency list: node -> Vec<(neighbor, weight)>.
        let n = n as usize;
        let mut adjacency: Vec<Vec<(usize, i32)>> = vec![Vec::new(); n + 1];
        for edge in &times {
            adjacency[edge[0] as usize].push((edge[1] as usize, edge[2]));
        }

        // Dijkstra from source node k.
        let mut dist = vec![i32::MAX; n + 1];
        dist[k as usize] = 0;

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((0, k as usize)));

        while let Some(Reverse((cost, node))) = heap.pop() {
            if cost > dist[node] {
                continue;
            }
            for &(neighbor, weight) in &adjacency[node] {
                let next = cost + weight;
                if next < dist[neighbor] {
                    dist[neighbor] = next;
                    heap.push(Reverse((next, neighbor)));
                }
            }
        }

        // The answer is the slowest arrival across all nodes (1..=n).
        let slowest = dist[1..=n].iter().copied().max().unwrap_or(i32::MAX);
        if slowest == i32::MAX {
            -1
        } else {
            slowest
        }
    }
}
