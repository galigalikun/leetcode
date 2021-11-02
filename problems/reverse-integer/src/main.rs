fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1534236469), 0);
}

pub struct Solution {}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let _x = match x
            .to_string()
            .as_str()
            .chars()
            .filter(|x| x != &'-')
            .rev()
            .collect::<String>()
            .parse::<i32>()
        {
            Ok(a) => a,
            Err(_) => 0,
        };

        if x > 0 {
            return _x;
        }
        return -1 * _x;
    }
}
