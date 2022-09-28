fn main() {
    assert_eq!(Solution::find_replace_string("abcd".to_string(), vec![0, 2], vec!["a".to_string(), "cd".to_string()], vec!["eee".to_string(), "ffff".to_string()]), "eeebffff");
    assert_eq!(Solution::find_replace_string("abcd".to_string(), vec![0, 2], vec!["ab".to_string(),"ec".to_string()], vec!["eee".to_string(),"ffff".to_string()]), "eeecd");
}

struct Solution{}
impl Solution {
    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {
        let mut ans = String::new();
        return ans;
    }
}
