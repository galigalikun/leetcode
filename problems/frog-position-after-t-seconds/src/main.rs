fn main() {
    assert_eq!(
        Solution::frog_position(
            7,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 7],
                vec![2, 4],
                vec![2, 6],
                vec![3, 5]
            ],
            2,
            4
        ),
        0.16666666666666666
    );
    assert_eq!(
        Solution::frog_position(
            7,
            vec![
                vec![1, 2],
                vec![1, 3],
                vec![1, 7],
                vec![2, 4],
                vec![2, 6],
                vec![3, 5]
            ],
            1,
            7
        ),
        0.3333333333333333
    );
}

struct Solution;
impl Solution {
    pub fn frog_position(n: i32, edges: Vec<Vec<i32>>, t: i32, target: i32) -> f64 {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[(edge[0] - 1) as usize].push(edge[1] - 1);
            graph[(edge[1] - 1) as usize].push(edge[0] - 1);
        }
        let mut visited = vec![false; n as usize];
        visited[0] = true;
        let mut queue = vec![(0, 1.0)];
        for _ in 0..t {
            let mut next = vec![];
            for (node, prob) in queue {
                let mut count = 0;
                for &nei in &graph[node as usize] {
                    if !visited[nei as usize] {
                        count += 1;
                    }
                }
                if count == 0 {
                    next.push((node, prob));
                } else {
                    for &nei in &graph[node as usize] {
                        if !visited[nei as usize] {
                            visited[nei as usize] = true;
                            next.push((nei, prob / count as f64));
                        }
                    }
                }
            }
            queue = next;
        }
        for (node, prob) in queue {
            if node == target - 1 {
                return prob;
            }
        }
        return 0.0;
    }
}
