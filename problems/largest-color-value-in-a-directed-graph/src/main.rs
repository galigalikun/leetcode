fn main() {
    assert_eq!(
        Solution::largest_path_value(
            "abaca".to_string(),
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
        ),
        3
    );
    assert_eq!(
        Solution::largest_path_value("a".to_string(), vec![vec![0, 0]]),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        let n = colors.len();
        if n == 0 {
            return 0;
        }
        let mut graph = vec![vec![]; n];
        let mut indegree = vec![0; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            indegree[v] += 1;
        }
        let mut queue = std::collections::VecDeque::new();
        let mut color_count = vec![vec![0; 26]; n];
        for i in 0..n {
            color_count[i][(colors.as_bytes()[i] - b'a') as usize] = 1;
            if indegree[i] == 0 {
                queue.push_back(i);
            }
        }
        let mut max_value = 0;
        while let Some(node) = queue.pop_front() {
            max_value = max_value.max(*color_count[node].iter().max().unwrap());
            for &neighbor in &graph[node] {
                for i in 0..26 {
                    color_count[neighbor][i] = color_count[neighbor][i].max(color_count[node][i]);
                }
                indegree[neighbor] -= 1;
                if indegree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }
        for &deg in &indegree {
            if deg > 0 {
                return -1; // Cycle detected
            }
        }
        if max_value == 0 {
            return -1; // No valid path found
        }
        max_value as i32
    }
}
