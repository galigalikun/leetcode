fn main() {
    assert_eq!(
        Solution::word_break(
            "catsanddog".to_string(),
            vec![
                "cat".to_string(),
                "cats".to_string(),
                "and".to_string(),
                "sand".to_string(),
                "dog".to_string()
            ]
        ),
        vec!["cat sand dog".to_string(), "cats and dog".to_string()]
    );

    assert_eq!(
        Solution::word_break(
            "pineapplepenapple".to_string(),
            vec![
                "apple".to_string(),
                "pen".to_string(),
                "applepen".to_string(),
                "pine".to_string(),
                "pineapple".to_string()
            ]
        ),
        vec![
            "pine apple pen apple".to_string(),
            "pine applepen apple".to_string(),
            "pineapple pen apple".to_string()
        ]
    );

    // assert_eq!(
    //     Solution::word_break(
    //         "catsandog".to_string(),
    //         vec![
    //             "cats".to_string(),
    //             "dog".to_string(),
    //             "sand".to_string(),
    //             "and".to_string(),
    //             "cat".to_string()
    //         ]
    //     ),
    //     vec![]
    // );
}

// https://programmerstart.com/article/4047826537/
pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    fn helper(
        result: &mut HashMap<String, Vec<String>>,
        s: String,
        word_dict: &Vec<String>,
    ) -> Vec<String> {
        if let Some(r) = result.get(&s) {
            if r.len() > 0 {
                return r.to_vec();
            }
        }
        if s.is_empty() {
            return vec!["".to_string()];
        }
        let mut res: Vec<String> = vec![];
        for w in word_dict {
            if w.len() > s.len() {
                continue;
            }
            if s[0..w.len()] != *w {
                continue;
            }
            let remain = Solution::helper(result, s[w.len()..].to_string(), word_dict);
            for r in remain {
                res.push(format!("{}{}{}", w, if r.is_empty() { "" } else { " " }, r));
            }
        }
        result.insert(s, res.clone());
        return res;
    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let mut word_set = word_dict;
        word_set.sort();
        return Solution::helper(&mut HashMap::new(), s, &word_set);
    }
}
