fn main() {
    assert_eq!(Solution::my_sqrt(4), 2);
    assert_eq!(Solution::my_sqrt(8), 2);
}

pub struct Solution {}
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        return (x as f64).sqrt() as i32;
    }
}
