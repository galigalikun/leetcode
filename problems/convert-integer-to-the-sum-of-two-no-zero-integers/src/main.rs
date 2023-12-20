fn main() {
    assert_eq!(Solution::get_no_zero_integers(2), vec![1, 1]);
    assert_eq!(Solution::get_no_zero_integers(11), vec![2, 9]);
}

struct Solution;
impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        for i in 1..n {
            if !format!("{}{}", i, n - i).contains("0") {
                return vec![i, n - i];
            }
        }
        return vec![];
    }
}
