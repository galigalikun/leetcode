fn main() {
    assert_eq!(
        Solution::minimum_abs_difference(vec![4, 2, 1, 3]),
        vec![[1, 2], [2, 3], [3, 4]]
    );
    assert_eq!(
        Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15]),
        vec![[1, 3]]
    );
    assert_eq!(
        Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27]),
        vec![[-14, -10], [19, 23], [23, 27]]
    );
}

struct Solution;
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let mut min = i32::MAX;
        let mut result = vec![];
        for i in 0..arr.len() - 1 {
            let diff = (arr[i] - arr[i + 1]).abs();
            if diff < min {
                min = diff;
                result.clear();
                result.push(vec![arr[i], arr[i + 1]]);
            } else if diff == min {
                result.push(vec![arr[i], arr[i + 1]]);
            }
        }
        return result;
    }
}
