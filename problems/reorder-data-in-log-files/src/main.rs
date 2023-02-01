fn main() {
    assert_eq!(Solution::reorder_log_files(vec!["dig1 8 1 5 1".to_string(),"let1 art can".to_string(),"dig2 3 6".to_string(),"let2 own kit dig".to_string(),"let3 art zero".to_string()]), vec!["let1 art can","let3 art zero","let2 own kit dig","dig1 8 1 5 1","dig2 3 6"]);
    assert_eq!(Solution::reorder_log_files(vec!["a1 9 2 3 1".to_string(),"g1 act car".to_string(),"zo4 4 7".to_string(),"ab1 off key dog".to_string(),"a8 act zoo".to_string()]), vec!["g1 act car","a8 act zoo","ab1 off key dog","a1 9 2 3 1","zo4 4 7"]);
}

struct Solution;
impl Solution {
    pub fn reorder_log_files(logs: Vec<String>) -> Vec<String> {
        return vec![];
    }
}
