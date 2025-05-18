fn main() {
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1]),
        2
    );
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000]),
        3
    );
    assert_eq!(
        Solution::maximum_element_after_decrementing_and_rearranging(vec![1, 2, 3, 4, 5]),
        5
    );
}

struct Solution;
impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        return arr.iter().enumerate().fold(0, |acc, (i, &x)| {
            let mut x = x;
            if i > 0 {
                x = x.min(acc + 1);
            }
            x
        });
    }
}
