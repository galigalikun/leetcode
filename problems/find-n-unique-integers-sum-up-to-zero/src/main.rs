fn main() {
    assert_eq!(Solution::sum_zero(5), vec![1, -1, 2, -2, 0]);
    assert_eq!(Solution::sum_zero(3), vec![1, -1, 0]);
    assert_eq!(Solution::sum_zero(1), vec![0]);
}

struct Solution;
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        if n > 0 {
            let mut result = vec![];
            for i in 1..=n / 2 {
                result.push(i);
                result.push(-i);
            }
            if n % 2 != 0 {
                result.push(0);
            }
            return result;
        }
        return vec![];
    }
}
