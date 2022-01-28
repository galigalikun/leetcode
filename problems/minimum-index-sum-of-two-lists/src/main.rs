fn main() {
    // assert_eq!(
    //     Solution::find_restaurant(
    //         vec![
    //             "Shogun".to_string(),
    //             "Tapioca Express".to_string(),
    //             "Burger King".to_string(),
    //             "KFC".to_string()
    //         ],
    //         vec![
    //             "Piatti".to_string(),
    //             "The Grill at Torrey Pines".to_string(),
    //             "Hungry Hunter Steakhouse".to_string(),
    //             "Shogun".to_string()
    //         ]
    //     ),
    //     vec!["Shogun"]
    // );
    // assert_eq!(
    //     Solution::find_restaurant(
    //         vec![
    //             "Shogun".to_string(),
    //             "Tapioca Express".to_string(),
    //             "Burger King".to_string(),
    //             "KFC".to_string()
    //         ],
    //         vec![
    //             "KFC".to_string(),
    //             "Shogun".to_string(),
    //             "Burger King".to_string()
    //         ]
    //     ),
    //     vec!["Shogun"]
    // );
    assert_eq!(
        Solution::find_restaurant(
            vec![
                "Shogun".to_string(),
                "Tapioca Express".to_string(),
                "Burger King".to_string(),
                "KFC".to_string()
            ],
            vec![
                "KFC".to_string(),
                "Burger King".to_string(),
                "Tapioca Express".to_string(),
                "Shogun".to_string()
            ]
        ),
        vec!["KFC", "Burger King", "Tapioca Express", "Shogun"]
    );
}

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_restaurant(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
        let mut map = HashMap::new();
        for i in 0..list1.len() {
            map.insert(&list1[i], i);
        }
        let mut ans: HashMap<usize, Vec<String>> = HashMap::new();
        let mut min = std::usize::MAX;
        for i in 0..list2.len() {
            if let Some(m) = map.get(&list2[i]) {
                let idx = m + i;
                min = std::cmp::min(min, idx);
                if let Some(a) = ans.get_mut(&idx) {
                    (*a).push(list2[i].clone());
                } else {
                    ans.insert(idx, vec![list2[i].clone()]);
                }
            }
        }
        return ans.get(&min).unwrap().to_vec();
    }
}
