fn main() {
    assert_eq!(Solution::min_number_operations(vec![1, 2, 3, 2, 1]), 3);
    assert_eq!(Solution::min_number_operations(vec![3, 1, 1, 2]), 4);
    assert_eq!(Solution::min_number_operations(vec![3, 1, 5, 4, 2]), 7);
}

struct Solution;
impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut res = target[0];
        for i in 1..target.len() {
            res += (target[i] - target[i - 1]).max(0);
        }
        res
    }
}
