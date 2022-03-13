fn main() {
    assert_eq!(Solution::is_power_of_four(16), true);
    assert_eq!(Solution::is_power_of_four(5), false);
    assert_eq!(Solution::is_power_of_four(1), true);
}

struct Solution {}
impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        if n == 0 {
            return false;
        } else if n == 1 {
            return true;
        } else if n == 4 {
            return true;
        } else if n % 4 == 0 {
            return Solution::is_power_of_four(n / 4);
        }
        return false;
    }
}
