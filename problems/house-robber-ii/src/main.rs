fn main() {
    assert_eq!(Solution::rob(vec![2, 3, 2]), 3);
    assert_eq!(Solution::rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(Solution::rob(vec![0]), 0);
    assert_eq!(Solution::rob(vec![1]), 1);
}

struct Solution {}
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        let mut tuple1 = (0, 0);
        let mut tuple2 = (0, 0);
        for i in 0..nums.len() {
            if i != nums.len() - 1 {
                tuple1 = (tuple1.1, std::cmp::max(tuple1.0 + nums[i], tuple1.1));
            }
            if i != 0 {
                tuple2 = (tuple2.1, std::cmp::max(tuple2.0 + nums[i], tuple2.1));
            }
        }

        return std::cmp::max(tuple1.1, tuple2.1);
    }
}
