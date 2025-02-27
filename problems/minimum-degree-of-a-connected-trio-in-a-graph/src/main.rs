fn main() {
    assert_eq!(
        Solution::min_trio_degree(
            6,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![3, 2],
                vec![4, 1],
                vec![5, 2],
                vec![3, 6]
            ]
        ),
        3
    );
    assert_eq!(
        Solution::min_trio_degree(
            7,
            vec![
                vec![1, 3],
                vec![4, 1],
                vec![4, 3],
                vec![2, 5],
                vec![5, 6],
                vec![6, 7],
                vec![7, 5],
                vec![2, 6]
            ]
        ),
        0
    );
    assert_eq!(
        Solution::min_trio_degree(3, vec![vec![3, 2], vec![2, 1]]),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn min_trio_degree(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut degree = vec![0; n as usize];
        let mut adj = vec![vec![false; n as usize]; n as usize];
        for edge in edges.iter() {
            let u = edge[0] as usize - 1;
            let v = edge[1] as usize - 1;
            degree[u] += 1;
            degree[v] += 1;
            adj[u][v] = true;
            adj[v][u] = true;
        }
        let mut ans = std::i32::MAX;
        for i in 0..n as usize {
            for j in i + 1..n as usize {
                if adj[i][j] {
                    for k in j + 1..n as usize {
                        if adj[i][k] && adj[j][k] {
                            ans = ans.min(degree[i] + degree[j] + degree[k] - 6);
                        }
                    }
                }
            }
        }
        if ans != std::i32::MAX {
            return ans;
        }
        return -1;
    }
}
