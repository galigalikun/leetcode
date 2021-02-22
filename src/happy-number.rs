fn main() {
    assert_eq!(Solution::is_happy(19), true);

    assert_eq!(Solution::is_happy(2), false);
}

pub struct Solution{}
impl Solution {
    fn helper(i: usize, n: i32) -> bool {
        let result = n.to_string().as_str().chars().map(|c| c.to_digit(10).unwrap().pow(2)).fold(0, |sum, a| sum + a);
        if i > 10 {
            return false;
        }
        if result == 1 {
            return true;
        }
        return Solution::helper(i+1, result as i32);
    }
    pub fn is_happy(n: i32) -> bool {
        return Solution::helper(0, n);
    }
}
