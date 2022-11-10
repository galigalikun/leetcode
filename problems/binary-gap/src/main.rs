fn main() {
    assert_eq!(Solution::binary_gap(22), 2);
    assert_eq!(Solution::binary_gap(8), 0);
}

struct Solution;
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut ans = 0;
        let mut dis = 0;
        for i in format!("{:b}", n).chars().enumerate() {
            if i.1 == '1' {
                ans = std::cmp::max(ans, i.0 - dis);
                dis = i.0;
            }
        }
        return ans as i32;
    }
}
