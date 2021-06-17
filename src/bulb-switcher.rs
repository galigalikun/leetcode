fn main() {
    assert_eq!(Solution::bulb_switch(3), 1);
    assert_eq!(Solution::bulb_switch(0), 0);
    assert_eq!(Solution::bulb_switch(1), 1);
}

pub struct Solution {}
impl Solution {
    pub fn bulb_switch(n: i32) -> i32 {
        return (n as f64).sqrt() as i32;
    }
}
