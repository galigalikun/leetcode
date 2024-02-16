fn main() {
    assert_eq!(
        Solution::rank_teams(vec![
            "ABC".to_string(),
            "ACB".to_string(),
            "ABC".to_string(),
            "ACB".to_string(),
            "ACB".to_string()
        ]),
        "ACB".to_string()
    );
    assert_eq!(
        Solution::rank_teams(vec!["WXYZ".to_string(), "XYZW".to_string()]),
        "XWYZ".to_string()
    );
    assert_eq!(
        Solution::rank_teams(vec!["ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()]),
        "ZMNAGUEDSJYLBOPHRQICWFXTVK".to_string()
    );
}

struct Solution;
impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let mut map = std::collections::HashMap::new();
        let mut len = 0;
        for vote in &votes {
            len = vote.len();
            for (i, c) in vote.chars().enumerate() {
                let counter = map.entry(c).or_insert(vec![0; len]);
                counter[i] += 1;
            }
        }
        let mut keys: Vec<char> = map.keys().map(|&x| x).collect();
        keys.sort_by(|a, b| {
            for i in 0..len {
                if map[a][i] != map[b][i] {
                    return map[b][i].cmp(&map[a][i]);
                }
            }
            a.cmp(b)
        });
        return keys.iter().collect();
    }
}
