fn main() {
    assert_eq!(
        Solution::dest_city(vec![
            vec!["London".to_string(), "New York".to_string()],
            vec!["New York".to_string(), "Lima".to_string()],
            vec!["Lima".to_string(), "Sao Paulo".to_string()]
        ]),
        "Sao Paulo"
    );
    assert_eq!(
        Solution::dest_city(vec![
            vec!["B".to_string(), "C".to_string()],
            vec!["D".to_string(), "B".to_string()],
            vec!["C".to_string(), "A".to_string()]
        ]),
        "A"
    );
    assert_eq!(
        Solution::dest_city(vec![vec!["A".to_string(), "Z".to_string()]]),
        "Z"
    );
}

struct Solution;
impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut map = std::collections::HashMap::new();
        for path in paths.clone() {
            map.insert(path[0].clone(), path[1].clone());
        }
        let mut city = paths[0][0].clone();
        while map.contains_key(&city) {
            city = map.get(&city).unwrap().clone();
        }
        return city;
    }
}
