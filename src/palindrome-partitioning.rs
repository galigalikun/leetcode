fn main() {
    assert_eq!(
        Solution::partition("aab".to_string()),
        vec![
            vec!["a".to_string(), "a".to_string(), "b".to_string()],
            vec!["aa".to_string(), "b".to_string()]
        ]
    );
    assert_eq!(
        Solution::partition("a".to_string()),
        vec![vec!["a".to_string()]]
    );
    assert_eq!(
        Solution::partition("efe".to_string()),
        vec![
            vec!["e".to_string(), "f".to_string(), "e".to_string()],
            vec!["efe".to_string()]
        ]
    );
    assert_eq!(
        Solution::partition("cdd".to_string()),
        vec![
            vec!["c".to_string(), "d".to_string(), "d".to_string()],
            vec!["c".to_string(), "dd".to_string()]
        ]
    );
    assert_eq!(
        Solution::partition("fff".to_string()),
        vec![
            vec!["f".to_string(), "f".to_string(), "f".to_string()],
            vec!["f".to_string(), "ff".to_string()],
            vec!["ff".to_string(), "f".to_string()],
            vec!["fff".to_string()]
        ]
    );
}

pub struct Solution {}
impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        for c in s.as_str().chars() {
            if let Some(f) = result.first() {
                if let Some(p) = f.iter().rposition(|x| x == &c.to_string()) {
                    let mut v = f[0..p].to_vec();
                    let mut s = f[p..].connect("");
                    s.push(c);
                    v.push(s);
                    result.push(v);
                    for i in 0..result.len() - 1 {
                        result[i].push(c.to_string());
                    }
                } else {
                    for i in 0..result.len() {
                        result[i].push(c.to_string());
                    }
                }
            } else {
                result.push(vec![c.to_string()]);
            }
        }
        return result;
    }
}
