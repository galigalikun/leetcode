fn main() {
    assert_eq!(Solution::shortest_path_all_keys(vec!["@.a..".to_string(),"###.#".to_string(),"b.A.B".to_string()]), 8);
    assert_eq!(Solution::shortest_path_all_keys(vec!["@..aA".to_string(),"..B#.".to_string(),"....b".to_string()]), 6);
    assert_eq!(Solution::shortest_path_all_keys(vec!["@Aa".to_string()]), -1);
}

struct Solution;
impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        return -1;
    }
}
