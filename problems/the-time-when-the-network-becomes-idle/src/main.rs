fn main() {
    assert_eq!(
        Solution::network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
        8
    );
    assert_eq!(
        Solution::network_becomes_idle(vec![vec![0, 1], vec![1, 2]], vec![0, 2, 1]),
        8
    );
}

struct Solution;
impl Solution {
    pub fn network_becomes_idle(edges: Vec<Vec<i32>>, patience: Vec<i32>) -> i32 {
        let mut graph = vec![vec![]; patience.len()];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        let mut dist = vec![-1; patience.len()];
        dist[0] = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(0);
        while let Some(u) = queue.pop_front() {
            for &v in &graph[u] {
                if dist[v] == -1 {
                    dist[v] = dist[u] + 1;
                    queue.push_back(v);
                }
            }
        }
        let mut ans = 0;
        for i in 1..patience.len() {
            let d = dist[i] * 2;
            let p = patience[i];
            let last = if d <= p { 0 } else { (d - 1) / p * p };
            ans = ans.max(last + d);
        }
        ans + 1
    }
}
