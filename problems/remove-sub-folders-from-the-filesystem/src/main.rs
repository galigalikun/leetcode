fn main() {
    assert_eq!(
        Solution::remove_subfolders(
            vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        ),
        vec!["/a", "/c/d", "/c/f"]
    );
    assert_eq!(
        Solution::remove_subfolders(
            vec!["/a", "/a/b/c", "/a/b/d"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        ),
        vec!["/a"]
    );
    assert_eq!(
        Solution::remove_subfolders(
            vec!["/a/b/c", "/a/b/ca", "/a/b/d"]
                .into_iter()
                .map(|x| x.to_string())
                .collect()
        ),
        vec!["/a/b/c", "/a/b/ca", "/a/b/d"]
    );
}

struct Solution;
impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let mut folder = folder;
        folder.sort();
        let mut result = vec![];
        let mut prev = String::new();
        for f in folder {
            if !f.starts_with(&prev) {
                result.push(f.clone());
                prev = f.clone();
                prev.push('/');
            }
        }
        return result;
    }
}
