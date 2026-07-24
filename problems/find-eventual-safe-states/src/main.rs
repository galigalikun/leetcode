fn main() {
    assert_eq!(Solution::eventual_safe_nodes(vec![vec![1,2],vec![2,3],vec![5],vec![0],vec![5],vec![],vec![]]), vec![2,4,5,6]);
    assert_eq!(Solution::eventual_safe_nodes(vec![vec![1,2,3,4],vec![1,2],vec![3,4],vec![0,4],vec![]]), vec![4]);
}

struct Solution{}
impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        fn is_safe(node: usize, graph: &[Vec<i32>], state: &mut [u8]) -> bool {
            if state[node] != 0 {
                return state[node] == 2;
            }

            state[node] = 1;
            for &next in &graph[node] {
                let next_node = next as usize;
                if state[next_node] == 1 || !is_safe(next_node, graph, state) {
                    return false;
                }
            }

            state[node] = 2;
            true
        }

        let n = graph.len();
        let mut state = vec![0_u8; n];
        let mut answer = Vec::new();

        for node in 0..n {
            if is_safe(node, &graph, &mut state) {
                answer.push(node as i32);
            }
        }

        answer
    }
}
