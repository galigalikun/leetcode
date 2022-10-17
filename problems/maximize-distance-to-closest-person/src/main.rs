fn main() {
    assert_eq!(Solution::max_dist_to_closest(vec![1,0,0,0,1,0,1]), 2);
    assert_eq!(Solution::max_dist_to_closest(vec![1,0,0,0]), 3);
    assert_eq!(Solution::max_dist_to_closest(vec![0,1]), 1);
}

struct Solution;
impl Solution {
    pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
        return 0;
    }
}
