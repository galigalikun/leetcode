fn main() {
    assert_eq!(Solution::check_if_exist(vec![10, 2, 5, 3]), true);
    assert_eq!(Solution::check_if_exist(vec![3, 1, 7, 11]), false);
}

struct Solution;
impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut map = std::collections::HashMap::new();
        for i in 0..arr.len() {
            map.insert(arr[i], i);
        }
        for i in 0..arr.len() {
            if let Some(&j) = map.get(&(arr[i] * 2)) {
                if i != j {
                    return true;
                }
            }
        }
        return false;
    }
}
