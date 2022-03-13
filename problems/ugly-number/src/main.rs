fn main() {
    assert_eq!(Solution::is_ugly(6), true);
    assert_eq!(Solution::is_ugly(8), true);
    assert_eq!(Solution::is_ugly(14), false);
    assert_eq!(Solution::is_ugly(1), true);
    assert_eq!(Solution::is_ugly(0), false);
}

struct Solution {}
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 1 {
            return true;
        } else if n == 0 {
            return false;
        }
        if n % 2 == 0 {
            return Solution::is_ugly(n / 2);
        } else if n % 3 == 0 {
            return Solution::is_ugly(n / 3);
        } else if n % 5 == 0 {
            return Solution::is_ugly(n / 5);
        }

        return false;
    }
}
