fn main() {
    assert_eq!(Solution::get_xor_sum(vec![1, 2, 3], vec![6, 5]), 0);
    assert_eq!(Solution::get_xor_sum(vec![12], vec![4]), 4);
}

struct Solution;
impl Solution {
    pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        return arr1.iter().fold(0, |acc, &x| acc ^ x) ^ arr2.iter().fold(0, |acc, &x| acc ^ x);
    }
}
