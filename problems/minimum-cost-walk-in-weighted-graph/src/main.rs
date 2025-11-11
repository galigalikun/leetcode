fn main() {
    assert_eq!(
        Solution::minimum_cost(
            5,
            vec![vec![0, 1, 7], vec![1, 3, 7], vec![1, 2, 1]],
            vec![vec![0, 3], vec![3, 4]],
        ),
        vec![1, -1]
    );
    assert_eq!(
        Solution::minimum_cost(
            3,
            vec![vec![0, 2, 7], vec![0, 1, 15], vec![1, 2, 6], vec![1, 2, 1]],
            vec![vec![1, 2]]
        ),
        vec![0]
    );
}

struct Solution;
impl Solution {
    pub fn minimum_cost(n: i32, edges: Vec<Vec<i32>>, query: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut dist = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dist[i][i] = 0;
        }
        for e in edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2];
            dist[u][v] = dist[u][v].min(w);
            dist[v][u] = dist[v][u].min(w);
        }
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != i32::MAX && dist[k][j] != i32::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }
        query
            .into_iter()
            .map(|q| {
                let u = q[0] as usize;
                let v = q[1] as usize;
                if dist[u][v] == i32::MAX {
                    -1
                } else {
                    dist[u][v]
                }
            })
            .collect()
    }
}
