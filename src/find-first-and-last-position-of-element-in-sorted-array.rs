fn main() {
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8),
        vec![3, 4]
    );
    assert_eq!(
        Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
        vec![-1, -1]
    );
    assert_eq!(Solution::search_range(vec![], 0), vec![-1, -1]);
}

pub struct Solution {}
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = vec![-1, -1];
        if let Some(p) = nums.iter().position(|&x| x == target) {
            result[0] = p as i32;
        }
        if let Some(p) = nums.iter().rposition(|&x| x == target) {
            result[1] = p as i32;
        }
        return result;
    }
}
