fn main() {
    assert_eq!(Solution::count_partitions(vec![10,10,3,7,6]), 4);
    assert_eq!(Solution::count_partitions(vec![1,2,2]), 0);
    assert_eq!(Solution::count_partitions(vec![2,4,6,8]), 3);
}

struct Solution;
impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        if nums.len() < 2 {
            return 0;
        }
        let total: i64 = nums.iter().map(|&x| x as i64).sum();
        if total % 2 != 0 {
            return 0;
        }
        (nums.len() as i32) - 1
    }
}
