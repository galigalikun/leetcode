fn main() {
    assert_eq!(
        Solution::count_restricted_paths(
            5,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![2, 3, 1],
                vec![1, 4, 2],
                vec![5, 2, 2],
                vec![3, 5, 1],
                vec![5, 4, 10]
            ]
        ),
        3
    );
    assert_eq!(
        Solution::count_restricted_paths(
            7,
            vec![
                vec![1, 3, 1],
                vec![4, 1, 2],
                vec![7, 3, 4],
                vec![2, 5, 3],
                vec![5, 6, 1],
                vec![6, 7, 2],
                vec![7, 5, 3],
                vec![2, 6, 4]
            ]
        ),
        1
    );
}

struct Solution;
impl Solution {
    pub fn count_restricted_paths(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for edge in edges {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            let w = edge[2];
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        let mut dist = vec![std::i32::MAX; n];
        dist[n - 1] = 0;
        let mut pq = std::collections::BinaryHeap::new();
        pq.push((0, n - 1));
        while let Some((d, u)) = pq.pop() {
            if d > dist[u] {
                continue;
            }
            for &(v, w) in &graph[u] {
                if dist[v] > dist[u] + w {
                    dist[v] = dist[u] + w;
                    pq.push((dist[v], v));
                }
            }
        }
        let mut dp = vec![-1; n];
        dp[n - 1] = 1;
        fn dfs(
            u: usize,
            graph: &Vec<Vec<(usize, i32)>>,
            dist: &Vec<i32>,
            dp: &mut Vec<i32>,
        ) -> i32 {
            if dp[u] != -1 {
                return dp[u];
            }
            let mut ans = 0;
            for &(v, _) in &graph[u] {
                if dist[u] > dist[v] {
                    ans += dfs(v, graph, dist, dp);
                    ans %= 1_000_000_007;
                }
            }
            dp[u] = ans;
            ans
        }
        dfs(0, &graph, &dist, &mut dp)
    }
}
