fn main() {
    assert_eq!(Solution::is_power_of_two(1), true);
    assert_eq!(Solution::is_power_of_two(16), true);
    assert_eq!(Solution::is_power_of_two(3), false);
    assert_eq!(Solution::is_power_of_two(4), true);
    assert_eq!(Solution::is_power_of_two(5), false);
    assert_eq!(Solution::is_power_of_two(0), false);
}

pub struct Solution {}
impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        if n == 1 {
            return true;
        }
        let mut div = n;
        loop {
            if div % 2 == 0 {
                div = div / 2;
                if div == 1 {
                    return true;
                }
            } else {
                break;
            }
        }
        return false;
    }
}
