fn main() {
    assert_eq!(
        Solution::simplify_path("/home/".to_string()),
        "/home".to_string()
    );
    assert_eq!(Solution::simplify_path("/../".to_string()), "/".to_string());
    assert_eq!(
        Solution::simplify_path("/a/./b/../../c/".to_string()),
        "/c".to_string()
    );
}

struct Solution {}
impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut pieces = path.split("/");
        let mut node = Vec::new();
        while let Some(p) = pieces.next() {
            if p.len() > 0 {
                if p == "." {
                } else if p == ".." {
                    if node.len() > 0 {
                        node.pop();
                    }
                } else {
                    node.push("/".to_owned() + p);
                }
            }
        }
        if node.len() == 0 {
            return "/".to_string();
        }
        return node.join("");
    }
}
