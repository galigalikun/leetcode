fn main() {
    assert_eq!(
        Solution::replace_elements(vec![17, 18, 5, 4, 6, 1]),
        vec![18, 6, 6, 6, 1, -1]
    );
    assert_eq!(Solution::replace_elements(vec![400]), vec![-1]);
}

struct Solution;
impl Solution {
    pub fn replace_elements(arr: Vec<i32>) -> Vec<i32> {
        let mut max = -1;
        let mut result = vec![];
        for i in (0..arr.len()).rev() {
            result.push(max);
            max = max.max(arr[i]);
        }
        result.reverse();
        return result;
    }
}
