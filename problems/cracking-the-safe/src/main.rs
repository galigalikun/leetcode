fn main() {
    assert_eq!(Solution::crack_safe(1, 2), "10");
    assert_eq!(Solution::crack_safe(2, 2), "01100");
}

use std::collections::HashSet;

struct Solution {}
impl Solution {
    /// Builds a de Bruijn sequence B(k, n): the shortest string in which every
    /// length-`n` string over the alphabet [0, k) appears as a substring.
    ///
    /// Each length-(n-1) string is a node; appending a digit `x` is an edge.
    /// A Hierholzer DFS over this graph walks every edge exactly once (an Euler
    /// circuit), and the post-order digits form the sequence.
    pub fn crack_safe(n: i32, k: i32) -> String {
        let n = n as usize;
        let k = k as usize;

        let start = "0".repeat(n - 1);
        let mut visited = HashSet::new();
        let mut result = String::new();

        Self::dfs(&start, k, &mut visited, &mut result);

        result.push_str(&start);
        result
    }

    fn dfs(node: &str, k: usize, visited: &mut HashSet<String>, result: &mut String) {
        for x in 0..k {
            let edge = format!("{node}{x}");
            if visited.insert(edge.clone()) {
                let next = &edge[1..];
                Self::dfs(next, k, visited, result);
                result.push_str(&x.to_string());
            }
        }
    }
}
