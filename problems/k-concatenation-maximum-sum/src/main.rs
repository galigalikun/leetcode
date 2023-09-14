fn main() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, 2], 3), 9);
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
    assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 7), 0);
}

struct Solution;
impl Solution {
    pub fn k_concatenation_max_sum(arr: Vec<i32>, k: i32) -> i32 {
        let a: i32 = arr.iter().sum();
        return a * k;
    }
}
