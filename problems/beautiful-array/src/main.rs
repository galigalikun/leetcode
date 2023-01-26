fn main() {
    assert_eq!(Solution::beautiful_array(4), vec![2,1,4,3]);
    assert_eq!(Solution::beautiful_array(5), vec![3,1,2,5,4]);
}

struct Solution;
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut ans = vec![0; n as usize];
        return ans;
    }
}
