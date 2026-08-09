fn main() {
    assert_eq!(Solution::find_replace_string("abcd".to_string(), vec![0, 2], vec!["a".to_string(), "cd".to_string()], vec!["eee".to_string(), "ffff".to_string()]), "eeebffff");
    assert_eq!(Solution::find_replace_string("abcd".to_string(), vec![0, 2], vec!["ab".to_string(),"ec".to_string()], vec!["eee".to_string(),"ffff".to_string()]), "eeecd");
}

struct Solution{}
impl Solution {
    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        let mut ops: Vec<(usize, &str, &str)> = indices
            .iter()
            .zip(sources.iter())
            .zip(targets.iter())
            .map(|((idx, source), target)| (*idx as usize, source.as_str(), target.as_str()))
            .collect();
        ops.sort_by_key(|(idx, _, _)| *idx);

        let n = s.len();
        let s_bytes = s.as_bytes();
        let mut ans = String::with_capacity(n);
        let mut pos = 0;

        for (idx, source, target) in ops {
            if idx > n {
                continue;
            }
            if pos < idx {
                ans.push_str(&s[pos..idx]);
                pos = idx;
            }
            if idx + source.len() <= n && &s_bytes[idx..idx + source.len()] == source.as_bytes() {
                ans.push_str(target);
                pos = idx + source.len();
            }
        }

        if pos < n {
            ans.push_str(&s[pos..]);
        }

        ans
    }
}
