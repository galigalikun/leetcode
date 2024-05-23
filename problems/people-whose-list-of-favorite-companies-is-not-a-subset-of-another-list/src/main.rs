fn main() {
    assert_eq!(
        Solution::people_indexes(vec![
            vec![
                "leetcode".to_string(),
                "google".to_string(),
                "facebook".to_string()
            ],
            vec!["google".to_string(), "microsoft".to_string()],
            vec!["google".to_string(), "facebook".to_string()],
            vec!["google".to_string()],
            vec!["amazon".to_string()]
        ]),
        vec![0, 1, 4]
    );
    assert_eq!(
        Solution::people_indexes(vec![
            vec![
                "leetcode".to_string(),
                "google".to_string(),
                "facebook".to_string()
            ],
            vec!["leetcode".to_string(), "amazon".to_string()],
            vec!["facebook".to_string(), "google".to_string()]
        ]),
        vec![0, 1]
    );
    assert_eq!(
        Solution::people_indexes(vec![
            vec!["leetcode".to_string()],
            vec!["google".to_string()],
            vec!["facebook".to_string()],
            vec!["amazon".to_string()]
        ]),
        vec![0, 1, 2, 3]
    );
}

struct Solution;
impl Solution {
    pub fn people_indexes(favorite_companies: Vec<Vec<String>>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        for (i, companies) in favorite_companies.iter().enumerate() {
            for company in companies.iter() {
                map.entry(company).or_insert(vec![]).push(i);
            }
        }
        let mut res = vec![];
        for (i, companies) in favorite_companies.iter().enumerate() {
            let mut set = std::collections::HashSet::new();
            for company in companies.iter() {
                for &j in map.get(company).unwrap() {
                    set.insert(j);
                }
            }
            if set.len() == companies.len() {
                res.push(i as i32);
            }
        }
        return res;
    }
}
