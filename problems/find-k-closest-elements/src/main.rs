fn main() {
    assert_eq!(Solution::find_closest_elements(vec![1,2,3,4,5], 4, 3), vec![1,2,3,4]);
    assert_eq!(Solution::find_closest_elements(vec![1,2,3,4,5], 4, -1), vec![1,2,3,4]);
}

struct Solution{}
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // a - x < b - x
        // 1 - 3 < 2 - 3
        // -2 < -1
        return vec![];
    }
}
