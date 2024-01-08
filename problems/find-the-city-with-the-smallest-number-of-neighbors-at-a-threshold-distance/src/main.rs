fn main() {
    assert_eq!(
        Solution::find_the_city(
            4,
            vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]],
            4
        ),
        3
    );
    assert_eq!(
        Solution::find_the_city(
            5,
            vec![
                vec![0, 1, 2],
                vec![0, 4, 8],
                vec![1, 2, 3],
                vec![1, 4, 2],
                vec![2, 3, 1],
                vec![3, 4, 1]
            ],
            2
        ),
        0
    );
}

struct Solution;
impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let mut ans = 0;
        let mut min = n;
        let mut graph = vec![vec![i32::MAX; n as usize]; n as usize];
        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            graph[u][v] = w;
            graph[v][u] = w;
        }
        for i in 0..n as usize {
            graph[i][i] = 0;
        }
        for k in 0..n as usize {
            for i in 0..n as usize {
                for j in 0..n as usize {
                    if graph[i][k] != i32::MAX && graph[k][j] != i32::MAX {
                        graph[i][j] = graph[i][j].min(graph[i][k] + graph[k][j]);
                    }
                }
            }
        }
        for i in 0..n as usize {
            let mut count = 0;
            for j in 0..n as usize {
                if graph[i][j] <= distance_threshold {
                    count += 1;
                }
            }
            if count <= min {
                min = count;
                ans = i;
            }
        }
        return ans as i32;
    }
}
