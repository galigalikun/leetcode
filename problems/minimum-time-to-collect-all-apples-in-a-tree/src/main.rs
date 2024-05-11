fn main() {
    assert_eq!(
        Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, true, true, false]
        ),
        8
    );
    assert_eq!(
        Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, true, false, false, true, false]
        ),
        6
    );
    assert_eq!(
        Solution::min_time(
            7,
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 4],
                vec![1, 5],
                vec![2, 3],
                vec![2, 6]
            ],
            vec![false, false, false, false, false, false, false]
        ),
        0
    );
    assert_eq!(
        Solution::min_time(
            4,
            vec![vec![0, 2], vec![0, 3], vec![1, 2]],
            vec![false, true, false, false]
        ),
        4
    );
}

struct Solution;
impl Solution {
    fn dfs(
        node: usize,
        graph: &Vec<Vec<usize>>,
        has_apple: &Vec<bool>,
        visited: &mut Vec<bool>,
    ) -> i32 {
        visited[node] = true;
        let mut res = 0;
        for &next in &graph[node] {
            if visited[next] {
                continue;
            }
            res += Self::dfs(next, graph, has_apple, visited);
        }
        if (res > 0 || has_apple[node]) && node != 0 {
            res += 2;
        }
        return res;
    }
    pub fn min_time(n: i32, edges: Vec<Vec<i32>>, has_apple: Vec<bool>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for e in edges {
            graph[e[0] as usize].push(e[1] as usize);
        }
        let mut visited = vec![false; n as usize];
        return Self::dfs(0, &graph, &has_apple, &mut visited);
    }
}
