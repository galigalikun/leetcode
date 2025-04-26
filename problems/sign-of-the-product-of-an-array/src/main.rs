fn main() {
    assert_eq!(Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]), 1);
    assert_eq!(Solution::array_sign(vec![1, 5, 0, 2, -3]), 0);
    assert_eq!(Solution::array_sign(vec![-1, 1, -1, 1, -1]), -1);
}

struct Solution;
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut sign = 1;
        for &num in nums.iter() {
            if num == 0 {
                return 0;
            } else if num < 0 {
                sign *= -1;
            }
        }
        if sign == -1 {
            return -1;
        }
        if sign == 1 {
            return 1;
        }
        return 0;
    }
}
