fn main() {
    assert_eq!(
        Solution::build_array(vec![1, 3], 3),
        vec!["Push", "Push", "Pop", "Push"]
    );
    assert_eq!(
        Solution::build_array(vec![1, 2, 3], 3),
        vec!["Push", "Push", "Push"]
    );
    assert_eq!(Solution::build_array(vec![1, 2], 4), vec!["Push", "Push"]);
}

struct Solution;
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut i = 1;
        for num in target {
            while i < num {
                result.push("Push".to_string());
                result.push("Pop".to_string());
                i += 1;
            }
            result.push("Push".to_string());
            i += 1;
        }
        return result;
    }
}
