fn main() {
    assert_eq!(Solution::count_operations(2, 3), 3);
    assert_eq!(Solution::count_operations(10, 10), 1);
}

struct Solution;
impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        return if num1 == 0 || num2 == 0 {
            0
        } else if num1 >= num2 {
            1 + Solution::count_operations(num1 - num2, num2)
        } else {
            1 + Solution::count_operations(num1, num2 - num1)
        };
    }
}
