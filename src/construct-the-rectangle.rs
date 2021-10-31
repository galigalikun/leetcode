fn main() {
    assert_eq!(Solution::construct_rectangle(4), vec![2, 2]);
    assert_eq!(Solution::construct_rectangle(37), vec![37,1]);
    assert_eq!(Solution::construct_rectangle(122122), vec![427, 286]);
    assert_eq!(Solution::construct_rectangle(2), vec![2, 1]);
}

pub struct Solution{}
impl Solution {
    pub fn construct_rectangle(area: i32) -> Vec<i32> {
        let s = (area as f64).sqrt() as i32;
        for n in s..area {
            if area %n == 0 {
                let w = area/n;
                return vec![std::cmp::max(w, n), std::cmp::min(w, n)];
            }
        }
        return vec![area, 1];
    }
}
