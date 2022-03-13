fn main() {
    assert_eq!(
        Solution::calc_equation(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()]
            ],
            vec![2.0, 3.0],
            vec![
                vec!["a".to_string(), "c".to_string()],
                vec!["b".to_string(), "a".to_string()],
                vec!["a".to_string(), "e".to_string()],
                vec!["a".to_string(), "a".to_string()],
                vec!["x".to_string(), "x".to_string()]
            ]
        ),
        vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000]
    );

    assert_eq!(
        Solution::calc_equation(
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "c".to_string()],
                vec!["bc".to_string(), "cd".to_string()]
            ],
            vec![1.5, 2.5, 5.0],
            vec![
                vec!["a".to_string(), "c".to_string()],
                vec!["c".to_string(), "b".to_string()],
                vec!["bc".to_string(), "cd".to_string()],
                vec!["cd".to_string(), "bc".to_string()]
            ]
        ),
        vec![3.75000, 0.40000, 5.00000, 0.20000]
    );

    assert_eq!(
        Solution::calc_equation(
            vec![vec!["a".to_string(), "b".to_string()]],
            vec![0.5],
            vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["b".to_string(), "a".to_string()],
                vec!["a".to_string(), "c".to_string()],
                vec!["x".to_string(), "y".to_string()]
            ]
        ),
        vec![0.50000, 2.00000, -1.00000, -1.00000]
    );
}

struct Solution {}
// https://www.tutorialcup.com/interview/algorithm/evaluate-division.htm
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    fn helper(
        visited: &mut HashSet<String>,
        graph: HashMap<String, HashMap<String, f64>>,
        from: &String,
        to: &String,
    ) -> f64 {
        if !graph.contains_key(from) {
            return -1.0;
        }

        let g = graph.get(from).unwrap();

        if let Some(&v) = g.get(to) {
            return v;
        }
        visited.insert(from.to_string());

        for (key, &value) in g {
            if !visited.contains(key) {
                let ans = Solution::helper(visited, graph.clone(), key, to);
                if ans != -1.0 {
                    return ans * value;
                }
            }
        }
        return -1.0;
    }
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph: HashMap<String, HashMap<String, f64>> = HashMap::new();
        for i in 0..equations.len() {
            let from = &equations[i][0];
            let to = &equations[i][1];
            if let Some(g) = graph.get_mut(from) {
                (*g).insert(to.to_string(), values[i]);
            } else {
                let mut map = HashMap::new();
                map.insert(to.to_string(), values[i]);
                graph.insert(from.to_string(), map);
            }

            if let Some(g) = graph.get_mut(to) {
                (*g).insert(from.to_string(), 1.0 / values[i]);
            } else {
                let mut map = HashMap::new();
                map.insert(from.to_string(), 1.0 / values[i]);
                graph.insert(to.to_string(), map);
            }
        }

        let mut ans = vec![0.0; queries.len()];
        for i in 0..queries.len() {
            let mut visited = HashSet::new();
            ans[i] = Solution::helper(&mut visited, graph.clone(), &queries[i][0], &queries[i][1]);
        }

        return ans;
    }
}
