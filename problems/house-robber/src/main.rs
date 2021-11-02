fn main() {
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(Solution::rob(vec![2, 1, 1, 2]), 4);
    assert_eq!(Solution::rob(vec![1, 3, 1, 3, 100]), 103);
}

pub struct Solution {}
// https://qiita.com/KueharX/items/ad01fc743042cf2fdb2a
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut tuple = (0, 0);
        for n in nums {
            tuple = (tuple.1, std::cmp::max(tuple.0 + n, tuple.1));
        }

        return tuple.1;
    }
}
