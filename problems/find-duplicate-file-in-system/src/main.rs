fn main() {
    assert_eq!(
        Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
            "root/c 3.txt(abcd)".to_string(),
            "root/c/d 4.txt(efgh)".to_string(),
            "root 4.txt(efgh)".to_string()
        ]),
        vec![
            vec!["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            vec!["root/a/1.txt", "root/c/3.txt"]
        ]
    );
    assert_eq!(
        Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efgh)".to_string(),
            "root/c 3.txt(abcd)".to_string(),
            "root/c/d 4.txt(efgh)".to_string()
        ]),
        vec![
            vec!["root/a/2.txt", "root/c/d/4.txt"],
            vec!["root/a/1.txt", "root/c/3.txt"]
        ]
    );
    assert_eq!(
        Solution::find_duplicate(vec![
            "root/a 1.txt(abcd) 2.txt(efsfgh)".to_string(),
            "root/c 3.txt(abdfcd)".to_string(),
            "root/c/d 4.txt(efggdfh)".to_string()
        ]),
        vec![]
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: HashMap<&str, Vec<String>> = HashMap::new();
        for i in 0..paths.len() {
            let lst = paths[i].split(" ").collect::<Vec<_>>();
            let dir = lst[0];
            for j in 1..lst.len() {
                if let Some(p) = lst[j].find(|c| c == '(') {
                    let key = &lst[j][p..];
                    let path = format!("{}/{}", dir, &lst[j][0..p]);
                    if let Some(a) = ans.get_mut(key) {
                        (*a).push(path);
                    } else {
                        ans.insert(key, vec![path]);
                    }
                }
            }
        }
        let mut result = ans.into_iter().map(|x| x.1).collect::<Vec<_>>();
        result.sort_by(|a, b| b.cmp(a));
        return result;
    }
}
