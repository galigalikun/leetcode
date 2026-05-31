fn main() {
    assert_eq!(
        Solution::max_distance(vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]]),
        4
    );
    assert_eq!(Solution::max_distance(vec![vec![1], vec![1]]), 0);
    assert_eq!(Solution::max_distance(vec![vec![1, 4], vec![0, 5]]), 4);
}

struct Solution;
impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let mut ans = 0;
        for arr in arrays {
            if arr[0] < min {
                min = arr[0];
            }
            if arr[arr.len() - 1] > max {
                max = arr[arr.len() - 1];
            }
            ans = ans.max((arr[arr.len() - 1] - min).max(max - arr[0]));
        }
        ans
    }
}
