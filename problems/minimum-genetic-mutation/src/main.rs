fn main() {
    assert_eq!(
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AACCGGTA".to_string(),
            vec!["AACCGGTA".to_string()]
        ),
        1
    );
    assert_eq!(
        Solution::min_mutation(
            "AACCGGTT".to_string(),
            "AAACGGTA".to_string(),
            vec![
                "AACCGGTA".to_string(),
                "AACCGCTA".to_string(),
                "AAACGGTA".to_string()
            ]
        ),
        2
    );
    assert_eq!(
        Solution::min_mutation(
            "AAAAACCC".to_string(),
            "AACCCCCC".to_string(),
            vec![
                "AAAACCCC".to_string(),
                "AAACCCCC".to_string(),
                "AACCCCCC".to_string()
            ]
        ),
        3
    );
    assert_eq!(
        Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), vec![]),
        -1
    );
    assert_eq!(
        Solution::min_mutation(
            "AAAACCCC".to_string(),
            "CCCCCCCC".to_string(),
            vec![
                "AAAACCCA".to_string(),
                "AAACCCCA".to_string(),
                "AACCCCCA".to_string(),
                "AACCCCCC".to_string(),
                "ACCCCCCC".to_string(),
                "CCCCCCCC".to_string(),
                "AAACCCCC".to_string(),
                "AACCCCCC".to_string()
            ]
        ),
        4
    );
}

struct Solution {}

// https://nataliekung.gitbook.io/ladder_code/lian-xi/minimum-genetic-mutation
use std::collections::HashMap;
use std::collections::HashSet;
impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        let chars = vec!['A', 'C', 'G', 'T'];
        let mut q = vec![start.clone()];
        let mut distance = HashMap::new();
        distance.insert(start, 0);
        let uk = bank.into_iter().collect::<HashSet<String>>();
        while !q.is_empty() {
            let s = q.remove(0);
            if s == end {
                return *distance.get(&s).unwrap_or(&0);
            }
            for i in 0..s.len() {
                for c in &chars {
                    let nw = format!("{}{}{}", &s[..i], c, &s[i + 1..]);
                    if None != uk.iter().find(|&x| x == &nw) && None == distance.get(&nw) {
                        distance.insert(nw.clone(), distance.get(&s).unwrap_or(&0) + 1);
                        q.push(nw);
                    }
                }
            }
        }
        return -1;
    }
}
