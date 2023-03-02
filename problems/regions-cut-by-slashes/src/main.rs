fn main() {
    assert_eq!(Solution::regions_by_slashes(vec![" /".to_string(),"/ ".to_string()]), 2);
    assert_eq!(Solution::regions_by_slashes(vec![" /".to_string(),"  ".to_string()]), 1);
    assert_eq!(Solution::regions_by_slashes(vec!["/\\".to_string(),"\\/".to_string()]), 5);
}

struct Solution;
impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        return 0;
    }
}
