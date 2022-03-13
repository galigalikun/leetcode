fn main() {
    assert_eq!(Solution::find_complement(5), 2);
    assert_eq!(Solution::find_complement(1), 0);
}

struct Solution {}
impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        return num ^ (2_i32.pow(format!("{:0>b}", num).len() as u32) - 1);
    }
}
