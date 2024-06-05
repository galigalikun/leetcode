fn main() {
    assert_eq!(
        Solution::min_reorder(
            6,
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]]
        ),
        3
    );
    assert_eq!(
        Solution::min_reorder(5, vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]]),
        2
    );
    assert_eq!(Solution::min_reorder(3, vec![vec![1, 0], vec![2, 0]]), 0);
}

struct Solution;
impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; n as usize];
        for c in connections {
            graph[c[0] as usize].push((c[1], true));
            graph[c[1] as usize].push((c[0], false));
        }
        let mut res = 0;
        let mut visited = vec![false; n as usize];
        let mut stack = vec![0];
        visited[0] = true;
        while !stack.is_empty() {
            let node = stack.pop().unwrap();
            for (next, is_reversed) in graph[node].iter() {
                if !visited[*next as usize] {
                    res += if *is_reversed { 1 } else { 0 };
                    visited[*next as usize] = true;
                    stack.push(*next as usize);
                }
            }
        }
        return res;
    }
}
