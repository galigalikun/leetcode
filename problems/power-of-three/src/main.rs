fn main() {
    assert_eq!(Solution::is_power_of_three(27), true);
    assert_eq!(Solution::is_power_of_three(0), false);
    assert_eq!(Solution::is_power_of_three(9), true);
    assert_eq!(Solution::is_power_of_three(45), false);
    assert_eq!(Solution::is_power_of_three(1), true);
}

struct Solution {}
impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 0 {
            return false;
        } else if n == 1 {
            return true;
        } else if n == 3 {
            return true;
        } else if n % 3 == 0 {
            return Solution::is_power_of_three(n / 3);
        }
        return false;
    }
}
