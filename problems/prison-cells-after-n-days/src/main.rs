fn main() {
    assert_eq!(Solution::prison_after_n_days(vec![0,1,0,1,1,0,0,1], 7), vec![0,0,1,1,0,0,0,0]);
    assert_eq!(Solution::prison_after_n_days(vec![1,0,0,1,0,0,1,0], 1000000000), vec![0,0,1,1,1,1,1,0]);
}

struct Solution;
impl Solution {
    pub fn prison_after_n_days(cells: Vec<i32>, n: i32) -> Vec<i32> {
        return vec![];
    }
}
