fn main() {
    assert_eq!(
        Solution::get_folder_names(vec![
            "pes".to_string(),
            "fifa".to_string(),
            "gta".to_string(),
            "pes(2019)".to_string()
        ]),
        vec!["pes", "fifa", "gta", "pes(2019)"]
    );
    assert_eq!(
        Solution::get_folder_names(vec![
            "gta".to_string(),
            "gta(1)".to_string().to_string(),
            "gta".to_string(),
            "avalon".to_string()
        ]),
        vec!["gta", "gta(1)", "gta(2)", "avalon"]
    );
    assert_eq!(
        Solution::get_folder_names(vec![
            "onepiece".to_string(),
            "onepiece(1)".to_string(),
            "onepiece(2)".to_string(),
            "onepiece(3)".to_string(),
            "onepiece".to_string()
        ]),
        vec![
            "onepiece",
            "onepiece(1)",
            "onepiece(2)",
            "onepiece(3)",
            "onepiece(4)"
        ]
    );
}

struct Solution;
impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut result = vec![];
        for name in names {
            if map.contains_key(&name) {
                let base_name = name.clone();
                let mut count = map.get_mut(&name).unwrap();
                let mut name_to_use = base_name.clone();
                while let std::collections::hash_map::Entry::Occupied(o) =
                    map.entry(name_to_use.clone())
                {
                    *count += 1;
                    name_to_use = format!("{}({})", name, count);
                }
                map.insert(name_to_use.clone(), 0);
                result.push(name_to_use);
            } else {
                map.insert(name.clone(), 0);
                result.push(name);
            }
        }
        return result;
    }
}
