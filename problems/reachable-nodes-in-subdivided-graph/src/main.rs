use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    assert_eq!(
        Solution::reachable_nodes(vec![vec![0, 1, 10], vec![0, 2, 1], vec![1, 2, 2]], 6, 3),
        13
    );
    assert_eq!(
        Solution::reachable_nodes(
            vec![vec![0, 1, 4], vec![1, 2, 6], vec![0, 2, 8], vec![1, 3, 1]],
            10,
            4
        ),
        23
    );
    assert_eq!(
        Solution::reachable_nodes(
            vec![
                vec![1, 2, 4],
                vec![1, 4, 5],
                vec![1, 3, 1],
                vec![2, 3, 4],
                vec![3, 4, 5]
            ],
            17,
            5
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        let n_usize = n as usize;
        let max_moves_i64 = i64::from(max_moves);

        let graph = edges.iter().fold(
            vec![Vec::<(usize, i64)>::new(); n_usize],
            |mut acc, edge| {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let w = i64::from(edge[2]) + 1;
                acc[u].push((v, w));
                acc[v].push((u, w));
                acc
            },
        );

        let inf = i64::MAX / 4;
        let mut dist = vec![inf; n_usize];
        let mut heap = BinaryHeap::new();
        dist[0] = 0;
        heap.push((Reverse(0_i64), 0_usize));

        while let Some((Reverse(current_dist), node)) = heap.pop() {
            if current_dist > dist[node] {
                continue;
            }

            graph[node]
                .iter()
                .copied()
                .for_each(|(next, weight)| {
                    let candidate = current_dist + weight;
                    if candidate < dist[next] {
                        dist[next] = candidate;
                        heap.push((Reverse(candidate), next));
                    }
                });
        }

        let reachable_original = dist
            .iter()
            .filter(|&&d| d <= max_moves_i64)
            .count() as i64;

        let reachable_subdivided = edges
            .iter()
            .map(|edge| {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                let cnt = i64::from(edge[2]);
                let left = (max_moves_i64 - dist[u]).max(0);
                let right = (max_moves_i64 - dist[v]).max(0);
                cnt.min(left + right)
            })
            .sum::<i64>();

        (reachable_original + reachable_subdivided) as i32
    }
}
