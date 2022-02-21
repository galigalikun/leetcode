fn main() {
    assert_eq!(Solution::exclusive_time(2, vec!["0:start:0".to_string(),"1:start:2".to_string(),"1:end:5".to_string(),"0:end:6".to_string()]), vec![3, 4]);
    assert_eq!(Solution::exclusive_time(1, vec!["0:start:0".to_string(),"0:start:2".to_string(),"0:end:5".to_string(),"0:start:6".to_string(),"0:end:6".to_string(),"0:end:7".to_string()]), vec![8]);
    assert_eq!(Solution::exclusive_time(2, vec!["0:start:0".to_string(),"0:start:2".to_string(),"0:end:5".to_string(),"1:start:6".to_string(),"1:end:6".to_string(),"0:end:7".to_string()]), vec![7, 1]);
}

struct Solution{}
impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        return vec![];
    }
}
