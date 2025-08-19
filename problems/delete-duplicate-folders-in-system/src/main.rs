fn main() {
    assert_eq!(
        Solution::delete_duplicate_folder(vec![
            vec!["a".to_string()],
            vec!["c".to_string()],
            vec!["d".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["d".to_string(), "a".to_string()]
        ]),
        vec![vec!["d"], vec!["d", "a"]]
    );
    assert_eq!(
        Solution::delete_duplicate_folder(vec![
            vec!["a".to_string()],
            vec!["c".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["a".to_string(), "b".to_string(), "x".to_string()],
            vec![
                "a".to_string(),
                "b".to_string(),
                "x".to_string(),
                "y".to_string()
            ],
            vec!["w".to_string()],
            vec!["w".to_string(), "y".to_string()]
        ]),
        vec![vec!["c"], vec!["c", "b"], vec!["a"], vec!["a", "b"]]
    );
}

struct Solution;
impl Solution {
    pub fn delete_duplicate_folder(paths: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut unique_paths = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for path in paths {
            if !seen.contains(&path) {
                seen.insert(path.clone());
                unique_paths.push(path);
            }
        }

        unique_paths
    }
}
