use std::collections::VecDeque;

fn main() {
    assert_eq!(
        Solution::is_bipartite(vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]]),
        false
    );
    assert_eq!(
        Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]]),
        true
    );
}

struct Solution {}

impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let n = graph.len();
        let mut colors = vec![0_i8; n];

        for start in 0..n {
            if colors[start] != 0 {
                continue;
            }

            colors[start] = 1;
            let mut queue = VecDeque::from([start]);

            while let Some(node) = queue.pop_front() {
                let current_color = colors[node];
                for &next in &graph[node] {
                    let next = next as usize;
                    if colors[next] == 0 {
                        colors[next] = -current_color;
                        queue.push_back(next);
                    } else if colors[next] == current_color {
                        return false;
                    }
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn odd_cycle_is_not_bipartite() {
        let graph = vec![vec![1, 2, 3], vec![0, 2], vec![0, 1, 3], vec![0, 2]];
        assert!(!Solution::is_bipartite(graph));
    }

    #[test]
    fn even_cycle_is_bipartite() {
        let graph = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];
        assert!(Solution::is_bipartite(graph));
    }

    #[test]
    fn disconnected_graph_is_bipartite_when_all_components_are_bipartite() {
        let graph = vec![vec![1], vec![0], vec![3], vec![2], vec![]];
        assert!(Solution::is_bipartite(graph));
    }
}
